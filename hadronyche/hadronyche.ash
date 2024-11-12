#!/bin/ash

# functions
cleanre () {
  rm -rf seritinous-cone &&
  git clone https://github.com/jpegleg/serotinous-cone
}

buildrepo () {
  cat > /etc/apk/repositories << EOF; $(echo)

https://dl-cdn.alpinelinux.org/alpine/v$(cut -d'.' -f1,2 /etc/alpine-release)/main/
https://dl-cdn.alpinelinux.org/alpine/v$(cut -d'.' -f1,2 /etc/alpine-release)/community/
https://dl-cdn.alpinelinux.org/alpine/edge/testing/

EOF

  apk update
  apk upgrade
  apk add docker podman
  service docker restart
  curl -sL https://raw.githubusercontent.com/slimtoolkit/slim/master/scripts/install-slim.sh | bash -
}

# main setup
apk update
apk upgrade
rustup update || curl https://sh.rustup.rs -sSf | sh -s -- -y
. "$HOME/.cargo/env"
WKDR="/home/hadronyche/workspace/tmp"
DATO="$(hostname)$(date +%Y%m%d%H%M%SXX)"
mkdir -p $WKDR 2>/dev/null
cd $WKDR || exit 1
docker image ls || grep alpine.*community /etc/apk/repositories || buildrepo
cargo install cross

# build serotinous cone binaries
git clone https://github.com/jpegleg/serotinous-cone || cleanre
cd serotinous-cone/morph_micro_template
cargo clippy | tee -a $WKDR/"$DATO"
cargo test | tee -a $WKDR/"$DATO"
cargo build --release | tee -a $WKDR/"$DATO"

# build OCI containers
## work around usual cross pathcp /tmp/slim-state/.slim-state/images/*/artifacts/*.json .
mkdir -p target/x86_64-unknown-linux-musl/release/
## work around fill static content
mkdir -p static/.well-known
echo TESTMORPH > static/index.html
cp target/release/morph-server target/x86_64-unknown-linux-musl/release/
docker build -t "localhost:5000/morph-test" . | tee -a $WKDR/"$DATO"
docker save -o morph-test.tar "localhost:5000/morph-test:latest"

# collect hashes
sha3sum target/release/morph-server | tee -a $WKDR/"$DATO"
sha3sum morph-test.tar | tee -a $WKDR/"$DATO"

##-----> start security CI section

# collection cargo lock files and use as SBOM
cp "$WKDR"/serotinous-cone/morph_micro_template/Cargo.lock  "$WKDR"/"$DATO"_SBOM_Cargo-lock.txt

# create seccomp json trace
slim build "localhost:5000/morph-test"
mkdir -p /home/hadronyche/workspace/"$DATO"
cp /tmp/slim-state/.slim-state/images/*/artifacts/*.json /home/hadronyche/workspace/"$DATO"/
sha3sum *.json | tee "$WKDR"/slim_report_checksums-"$DATO".txt

# yara-x scan goes here, TODO

##----------------> wrap up
cp "$WKDR"/serotinous-cone/morph_micro_template/*.tar /home/hadronyche/workspace/"$DATO"/
cp "$WKDR"/serotinous-cone/morph_micro_template/Cargo.toml /home/hadronyche/workspace/"$DATO"/
cp "$WKDR"/serotinous-cone/morph_micro_template/slim_report_checksums-"$DATO".txt /home/hadronyche/workspace/"$DATO"/
rm -rf "$WKDR"
