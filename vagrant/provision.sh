#!/usr/bin/env bash

# This script gets executed as the `vagrant` user when you run `vagrant up`, and
# sets up a minimal configuration necessary to run unprivileged containers
# through lxc.
#
# Source:
# http://nik90.com/fiddling-around-with-lxc-containers/

sudo add-apt-repository -y ppa:ubuntu-lxc/lxc-stable
sudo apt-get update

# Set up lxc
sudo apt-get -qq install lxc-dev systemd-services uidmap
sudo usermod --add-subuids 100000-165536 $USER
sudo usermod --add-subgids 100000-165536 $USER
sudo chmod +x $HOME

mkdir -p ~/.config/lxc/
cat << EOF > ~/.config/lxc/default.conf
lxc.network.type = veth
lxc.network.link = lxcbr0
lxc.network.flags = up
lxc.network.hwaddr = 00:16:3e:xx:xx:xx
lxc.id_map = u 0 100000 65536
lxc.id_map = g 0 100000 65536
EOF

echo "$USER veth lxcbr0 10" | sudo tee -a /etc/lxc/lxc-usernet

# Install rust
curl -sf https://static.rust-lang.org/rustup.sh > rustup.sh
chmod +x rustup.sh
sudo ./rustup.sh -y
