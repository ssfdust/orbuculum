#!/bin/sh
sed -i '/\[powertools\]/,/enabled/s/enabled=0/enabled=1/' /etc/yum.repos.d/Rocky-PowerTools.repo
sed -e 's|^mirrorlist=|#mirrorlist=|g' \
    -e 's|^#baseurl=http://dl.rockylinux.org/$contentdir|baseurl=https://mirrors.nju.edu.cn/rocky|g' \
    -i.bak \
    /etc/yum.repos.d/Rocky-AppStream.repo \
    /etc/yum.repos.d/Rocky-BaseOS.repo \
    /etc/yum.repos.d/Rocky-Extras.repo \
    /etc/yum.repos.d/Rocky-PowerTools.repo
if command -v dnf; then
    dnf makecache
    dnf update -y
else
    microdnf makecache
    microdnf update -y
fi
