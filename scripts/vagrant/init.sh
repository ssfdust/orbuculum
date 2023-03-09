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

# Migrate to systemd name policy
sed -i -e 's/\s*biosdevname=0\s*//' -e 's/\s*net.ifnames=0\s*//' /etc/default/grub
grub2-mkconfig -o /boot/grub2/grub.cfg

# Network Card Configuration
mv /etc/sysconfig/network-scripts/ifcfg-eth1 /etc/sysconfig/network-scripts/ifcfg-enp1s4
sed -i 's/eth1/enp1s4/' /etc/sysconfig/network-scripts/ifcfg-enp1s4
mv /etc/sysconfig/network-scripts/ifcfg-eth2 /etc/sysconfig/network-scripts/ifcfg-enp1s5
sed -i 's/eth2/enp1s5/' /etc/sysconfig/network-scripts/ifcfg-enp1s5
mv /etc/sysconfig/network-scripts/ifcfg-eth3 /etc/sysconfig/network-scripts/ifcfg-enp1s6
echo "IPV6_AUTOCONF=no" >> /etc/sysconfig/network-scripts/ifcfg-enp1s5

# Install starship
dnf copr enable -y atim/starship 
dnf install -y starship
echo 'eval "$(starship init bash)"' | tee -a /home/vagrant/.bashrc

# create startship configuration
install -d -o vagrant -g vagrant /home/vagrant/.config
cat > /home/vagrant/.config/starship.toml <<EOF
format = """[\uE0B6](fg:#1C4961)[\$directory](bg:#1C4961)[\uE0B0](fg:#1C4961 bg:#2F79A1)\$git_branch[\uE0B0](fg:#2F79A1 bg:#3A95C7)\$git_status[\uE0B0](#3A95C7 bg:#40A9E0)\$cmd_duration[\uE0B0](#40A9E0 bg:none) \$all\$character"""

right_format = ""

add_newline = false

[directory]
style = "bg:#1C4961 fg:white"

[git_branch]
format = "[ \$symbol\$branch ](\$style)"
style = "bg:#2F79A1 fg:white"

[git_status]
format = "[ \$all_status\$ahead_behind ](\$style)"
style = "bg:#3A95C7 fg:white"

[cmd_duration]
disabled = false
format = "[ took \$duration ](\$style)"
show_milliseconds = true
style = "bg:#40A9E0 fg:white"
EOF

touch /etc/.initilized
