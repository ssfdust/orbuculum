Vagrant.configure("2") do |config|
  config.vm.box = "roboxes/rocky9"
  config.vm.guest = :rocky
  config.vm.synced_folder ".", "/vagrant", disabled: true
  config.vm.synced_folder "./", "/home/vagrant/workspace", type: "nfs", nfs_udp: false, nfs_version: "4.2"
  config.vm.synced_folder ENV["HOME"] + "/.cargo", "/home/vagrant/.cargo", type: "nfs", nfs_udp: false, nfs_version: "4.2"
  config.vm.network :private_network,
    :ip => "19.94.9.11",
    :libvirt__dhcp_enabled => false,
    :libvirt__forward_mode => "veryisolated",
    :libvirt__network_name => "vagrant-pri",
    :mac => "52:54:5e:13:7f:43",
    :bus => "1",
    :slot => "4"
  config.vm.network :private_network,
    :ip => "fe80::5054:ff:fe70:732e",
    :netmask => "64",
    :mac => "52:54:5e:13:7f:44",
    :libvirt__dhcp_enabled => false,
    :libvirt__forward_mode => "veryisolated",
    :libvirt__network_name => "vagrant-v6",
    :bus => "1",
    :slot => "5"
  config.vm.network :private_network,
    :ip => "19.96.4.12",
    :mac => "52:54:5e:13:7f:45",
    :libvirt__dhcp_enabled => false,
    :libvirt__forward_mode => "veryisolated",
    :libvirt__network_name => "vagrant-pri",
    :bus => "1",
    :slot => "6"
  config.vm.provider :libvirt do |libvirt|
    # CPU 核心数
    libvirt.cpus = 16
    # 内存大小
    libvirt.memory = 8192
    # 存储池名称
    libvirt.storage_pool_name = "virtual"
    # 使用系统接口
    libvirt.uri = "qemu:///system"
    # 默认网络修改
    libvirt.management_network_name = "vagrant-trusted"
    # 阻止自定义网络被删除
    libvirt.management_network_keep = false
    # libvirt.memorybacking :access, :mode => "shared"
  end
  # config.vm.synced_folder "./", "/home/vagrant/workspace", type: "virtiofs"
  # config.vm.synced_folder ENV["HOME"] + "/.cargo", "/home/vagrant/.cargo", type: "virtiofs"
  config.vm.provision "shell", inline: "bash /home/vagrant/workspace/scripts/vagrant/init.sh"
  config.trigger.after :provision do |trigger|
    trigger.info = "Adding kitty terminfo..."
    trigger.run = {inline: "bash -c 'infocmp -a xterm-kitty | vagrant ssh -c \"tic -x -o \~/.terminfo /dev/stdin\" 2>/dev/null'"}
  end
end
