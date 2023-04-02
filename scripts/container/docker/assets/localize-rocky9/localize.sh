#!/bin/sh
sed -i '/\[crb\]/,/enabled/s/enabled=0/enabled=1/' /etc/yum.repos.d/rocky.repo
sed -e 's|^mirrorlist=|#mirrorlist=|g' \
    -e 's|^#baseurl=http://dl.rockylinux.org/$contentdir|baseurl=https://mirrors.nju.edu.cn/rocky|g' \
    -i.bak \
    /etc/yum.repos.d/*.repo
dnf makecache
dnf update -y
