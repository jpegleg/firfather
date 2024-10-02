#!/bin/ash

sha256sum m.tar || exit 1
scp -P "$2" m.tar "$1" /root/m.tar || exit 1
cone --target "$1" --port "$2"
