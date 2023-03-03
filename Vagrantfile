Vagrant.configure("2") do |config|
  config.vm.box = "roboxes/rocky9"
  config.vm.synced_folder ".", "/vagrant", disabled: true
  config.vm.synced_folder "./", "/home/vagrant/workspace", type: "nfs", nfs_udp: false, nfs_version: "4.2"
  config.vm.synced_folder ENV["HOME"] + "/.cargo", "/home/vagrant/.cargo", type: "nfs", nfs_udp: false, nfs_version: "4.2"
  config.vm.network "private_network", type: "dhcp"
  config.vm.network "private_network", ip: "192.168.50.4"
  config.vm.provider :libvirt do |libvirt|
    # CPU 核心数
    libvirt.cpus = 16
    # 内存大小
    libvirt.memory = 16384
    # 存储池名称
    libvirt.storage_pool_name = "virtual"
    # 使用系统接口
    libvirt.uri = "qemu:///system"
    # 默认网络修改
    libvirt.management_network_name = "vagrant-virbr2"
    # 阻止自定义网络被删除
    libvirt.management_network_keep = true
  end
  config.vm.provision "shell", inline: "bash /home/vagrant/workspace/scripts/vagrant/init.sh"
  config.trigger.after :provision do |trigger|
    trigger.info = "Adding kitty terminfo.."
    trigger.run = {inline: "bash -c 'infocmp -a xterm-kitty | vagrant ssh -c \"tic -x -o \~/.terminfo /dev/stdin\" 2>/dev/null || true'"}
  end
end
