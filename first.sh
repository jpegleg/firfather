#!/bin/ash
apk update
apk upgrade
# alpine linux packages for making alpine packages
# plus some administrative tools such as vim and tmux
apk add git vim tmux wget curl python3 perl \
  cmake gcc clang openssl htop bash fish \
  alpine-sdk apk-tools apk-tools-dev apk-tools-static
# comment out the following line if a different motd is desired
grep "^Linux.*Alpine " /etc/motd || echo -e "built from Alpine Linux version: $(cat /etc/alpine-release) \nwith kernel version $(uname -a)\n" >> /etc/motd
