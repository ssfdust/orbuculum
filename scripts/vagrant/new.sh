mv /etc/sysconfig/network-scripts/ifcfg-eth1 /etc/sysconfig/network-scripts/ifcfg-enp1s4
sed -i 's/eth1/enp1s4/' /etc/sysconfig/network-scripts/ifcfg-enp1s4
mv /etc/sysconfig/network-scripts/ifcfg-eth2 /etc/sysconfig/network-scripts/ifcfg-enp1s5
sed -i 's/eth2/enp1s5/' /etc/sysconfig/network-scripts/ifcfg-enp1s5
