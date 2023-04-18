Orbuculum
-------------------

A system manager written in rust.

Development
===================
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
    -v /run/udev:/run/udev \
    -v /var/run/dbus/system_bus_socket:/var/run/dbus/system_bus_socket \
    -v /path/to/nic.rules:/etc/orbuculum/nic.rules:ro \
    docker.io/ssfdust/orbuculum:v0.0.1-alphav1-rockylinux-8-amd64
```

#### Rockylinux 9

```bash
sudo docker run --name orbuculum -d \
    --replace \
    --network=host \
    --privileged \
    -v /run/udev:/run/udev \
    -v /var/run/dbus/system_bus_socket:/var/run/dbus/system_bus_socket \
    -v /path/to/nic.rules:/etc/orbuculum/nic.rules:ro \
    docker.io/ssfdust/orbuculum:v0.0.1-alphav1-rockylinux-9-amd64
```

### open Euler 22.03 LTS

```bash
sudo docker run --name orbuculum -d \
    --replace \
    --network=host \
    --privileged \
    -v /run/udev:/run/udev \
    -v /var/run/dbus/system_bus_socket:/var/run/dbus/system_bus_socket \
    -v /path/to/nic.rules:/etc/orbuculum/nic.rules:ro \
    docker.io/ssfdust/orbuculum:v0.0.1-alphav1-openeuler-22.03-lts-amd64
```

### Testing

#### Rockylinux 8

```bash
sudo docker run --name orbuculum -d \
    --replace \
    --network=host \
    --privileged \
    -v /run/udev:/run/udev \
    -v /var/run/dbus/system_bus_socket:/var/run/dbus/system_bus_socket \
    docker.io/ssfdust/orbuculum:v0.0.1-alphav1-rockylinux-8-no-initialize-amd64
```

#### Rockylinux 9

```bash
sudo docker run --name orbuculum -d \
    --replace \
    --network=host \
    --privileged \
    -v /run/udev:/run/udev \
    -v /var/run/dbus/system_bus_socket:/var/run/dbus/system_bus_socket \
    docker.io/ssfdust/orbuculum:v0.0.1-alphav1-rockylinux-9-no-initialize-amd64
```

### open Euler 22.03 LTS

```bash
sudo docker run --name orbuculum -d \
    --replace \
    --network=host \
    --privileged \
    -v /run/udev:/run/udev \
    -v /var/run/dbus/system_bus_socket:/var/run/dbus/system_bus_socket \
    docker.io/ssfdust/orbuculum:v0.0.1-alphav1-openeuler-22.03-lts-no-initialize-amd64
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
