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
