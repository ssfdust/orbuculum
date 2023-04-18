#!/bin/sh
sed -e 's|^baseurl=http://repo.openeuler.org/|baseurl=https://mirrors.tuna.tsinghua.edu.cn/openeuler/|g' \
         -i.bak \
         /etc/yum.repos.d/openEuler.repo
yum makecache
yum update -y
