#!/bin/ash
apk update
apk upgrade
# alpine linux packages for making alpine packages
# plus some administrative tools such as vim and tmux
apk add git vim tmux wget curl python3 perl \
  cmake gcc clang openssl htop bash fish \
  alpine-sdk apk-tools apk-tools-dev apk-tools-static
