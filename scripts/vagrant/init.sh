#!/bin/sh
set -e
[ -f /etc/.initilized ] && exit 0

# Prepare environment
sed -i '/\[crb\]/,/enabled/s/enabled=0/enabled=1/' /etc/yum.repos.d/rocky.repo
sed -e 's|^mirrorlist=|#mirrorlist=|g' \
    -e 's|^#baseurl=http.://dl.rockylinux.org/$contentdir|baseurl=https://mirrors.nju.edu.cn/rocky|g' \
    -i.bak \
    /etc/yum.repos.d/rocky*.repo
sed -E 's|^[#]?baseurl=http.://download.example/pub|baseurl=https://mirrors.nju.edu.cn|' \
    -i.bak /etc/yum.repos.d/epel.repo
dnf makecache
dnf update -y
dnf install -y NetworkManager-libnm-devel systemd-devel git protobuf-devel gcc llvm jq

# Install rust
runuser -u vagrant -- /home/vagrant/.cargo/bin/rustup default nightly
echo "source ~/.cargo/env" | tee -a /home/vagrant/.bashrc
echo "export CARGO_TARGET_DIR=/home/vagrant/target" | tee -a /home/vagrant/.bashrc

touch /etc/.initilized
