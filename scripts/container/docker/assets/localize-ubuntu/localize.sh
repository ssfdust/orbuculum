#!/bin/sh
sed -i 's@//.*archive.ubuntu.com@//mirrors.nju.edu.cn@g' /etc/apt/sources.list
apt update -y
apt upgrade -y
