Orbuculum
===================

A system manager written in rust.

Prepare
-------------------
**Download assets/examples/rules/prod.rules to /etc/orbuculum/default.rules**

Network
-------------------
### Persistent Network Configuration

It solves the problem of mismatched configuraions because of hotplugging network
cards. For example, you have four network cards, which are eth0, eth1, eth2, eth3
Assuming there are four network cards, eth0, eth1, eth2, and eth3. and there are
two unused ethernet slots on the motherboard located in the front. If you insert
a new network card, the original eth0 card may become eth1. However, the
configuration of the previously designated eth0 card remains unchanged.

The network module resolves this issue by adapting to systemd's predictable
network interface names or by binding to MAC addresses, default to network
interface names.

#### Example for binding to mac address
```rhai
fn modify_connections(devices, device_type) {
    let new_devices = [];
    if device_type == "Ethernet" {
        for (device, idx) in devices {
            device["name"] = device["mac"];
            new_devices.push(device);
        }
    }

    return new_devices;
}
```

Development
-------------------
Vagrant
-------------------

### Install libvirt plugin

```bash
vagrant plugin install vagrant-libvirt
vagrant up --provider libvirt
vagrant ssh
cd workspace
```

After the first boot, it should be manually reloaded via `Vagrant reload`.

Docker
--------------------

### Deploy

#### Rockylinux 8

```bash
sudo docker run --name orbuculum -d \
    --replace \
    --network=host \
    --privileged \
    --entrypoint /init \
    -v /var/log:/var/log \
    -v /run/udev:/run/udev \
    -v /var/run/dbus/system_bus_socket:/var/run/dbus/system_bus_socket \
    -v /etc/orbuculum/nic.rules:/etc/orbuculum/nic.rules:ro \
    docker.io/ssfdust/orbuculum:v0.0.1-alphav1-rockylinux-8-amd64-r1
```

#### Rockylinux 9

```bash
sudo docker run --name orbuculum -d \
    --replace \
    --network=host \
    --privileged \
    --entrypoint /init \
    -v /var/log:/var/log \
    -v /run/udev:/run/udev \
    -v /var/run/dbus/system_bus_socket:/var/run/dbus/system_bus_socket \
    -v /etc/orbuculum/nic.rules:/etc/orbuculum/nic.rules:ro \
    docker.io/ssfdust/orbuculum:v0.0.1-alphav1-rockylinux-9-amd64-r1
```

#### Debian 11

```bash
sudo docker run --name orbuculum -d \
    --replace \
    --network=host \
    --privileged \
    --entrypoint /init \
    -v /var/log:/var/log \
    -v /run/udev:/run/udev \
    -v /var/run/dbus/system_bus_socket:/var/run/dbus/system_bus_socket \
    -v /etc/orbuculum/nic.rules:/etc/orbuculum/nic.rules:ro \
    docker.io/ssfdust/orbuculum:v0.0.1-alphav1-debian-11-amd64-r1
```

### Testing

#### Rockylinux 8

```bash
sudo docker run --name orbuculum -d \
    --replace \
    --network=host \
    --privileged \
    --entrypoint /init \
    -v /var/log:/var/log \
    -v /run/udev:/run/udev \
    -v /var/run/dbus/system_bus_socket:/var/run/dbus/system_bus_socket \
    docker.io/ssfdust/orbuculum:v0.0.1-alphav1-rockylinux-8-no-initialize-amd64-r1
```

#### Rockylinux 9

```bash
sudo docker run --name orbuculum -d \
    --replace \
    --network=host \
    --privileged \
    --entrypoint /init \
    -v /var/log:/var/log \
    -v /run/udev:/run/udev \
    -v /var/run/dbus/system_bus_socket:/var/run/dbus/system_bus_socket \
    docker.io/ssfdust/orbuculum:v0.0.1-alphav1-rockylinux-9-no-initialize-amd64-r1
```

#### Debian 11

```bash
sudo docker run --name orbuculum -d \
    --replace \
    --network=host \
    --privileged \
    --entrypoint /init \
    -v /var/log:/var/log \
    -v /run/udev:/run/udev \
    -v /var/run/dbus/system_bus_socket:/var/run/dbus/system_bus_socket \
    docker.io/ssfdust/orbuculum:v0.0.1-alphav1-debian-11-no-initialize-amd64-r1
```

### Development

1. Make sure docker is installed.
2. If you are behind a proxy, modify `scripts/Dockerfile.dev` and `scripts/cargo-docker` to set proxy

```bash
cd scripts
bash build-dev-docker.sh
source cargo-docker
cd ..
xcargo build
```

### About the network cards
The second network card is used for ipv4 testing, the third network card is used
for test ipv6 settings, the last is used for modifiction.
