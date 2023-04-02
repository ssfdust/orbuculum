#!/bin/sh
container=$(buildah from docker.io/library/rockylinux:9)
buildah add $container assets /
buildah run --network=host $container -- bash /localize-rocky9/localize.sh
buildah run --network=host $container -- bash -c "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -y"
buildah run --network=host $container -- dnf install -y NetworkManager-libnm-devel systemd-devel git protobuf-devel gcc llvm jq
buildah run --network=host $container -- dnf clean all
buildah run --network=host $container -- cp /localize-rocky9/config /root/.cargo/config
buildah config --env PATH=/root/.cargo/bin:$PATH $container
buildah commit $container docker.io/ssfdust/orbuculum-dev:latest
buildah rm $container
