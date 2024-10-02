#!/bin/ash
# See https://github.com/jpegleg/firfather for more information

yes | curl -sfL https://get.k3s.io | INSTALL_K3S_EXEC="server  --secrets-encryption --disable=metrics-server" sh -

k3s ctr image import /root/m.tar || echo "/root/m.tar not found!"

mkdir -p /srv/persist/TEMPLATE/.well-known/acme-challenge

mkdir -p /srv/persist/TEMPLATE-cert /srv/persist/TEMPLATE-key

chmod 600 /srv/persist/*key /srv/persist/*cert || echo "error on chmod of directories for TLS cert and key"

mv /root/TEMPLATE.tgz /srv/persist/ &&
cd /srv/persist/ && 
cp TEMPLATE.tgz TEMPLATE &&
cd TEMPLATE && 
tar xzvf TEMPLATE.tgz && 
rm TEMPLATE.tgz
