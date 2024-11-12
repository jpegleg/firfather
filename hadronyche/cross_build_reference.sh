#!/bin/bash
# cross build (such as would be run from a Debian system, etc etc)

# rustup update
rustup update || curl https://sh.rustup.rs -sSf | sh -s -- -y
. "$HOME/.cargo/env"
WKDR="/home/hadronyche/workspace/tmp"
DATO="$(hostname)$(date +%Y%m%d%H%M%SXX)"
mkdir -p $WKDR 2>/dev/null
cd $WKDR || exit 1
touch $DATO
cargo install cross

# build serotinous cone web server base template
git clone https://github.com/jpegleg/serotinous-cone || cleanre
cd serotinous-cone/morph_micro_template
cargo clippy | tee -a "$DATO"
cross build --target x86_64-unknown-linux-musl --release
