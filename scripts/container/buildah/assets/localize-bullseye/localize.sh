sed -i 's/deb.debian.org/mirrors.nju.edu.cn/g' /etc/apt/sources.list
apt update -y
apt upgrade -y
