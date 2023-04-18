#!/bin/sh
S6_OVERLAY_VERSION=3.1.4.2

projectdir="$(cd `pwd`;cd ../../..;pwd)"

container=$(buildah from docker.io/library/openeuler-22.03-lts:latest)
commit_container=$(buildah from docker.io/library/openeuler-22.03-lts:latest)

container_mnt=$(buildah mount $container)

buildah add $container assets /
buildah add $commit_container assets /

buildah run --network=host $container -- bash /localize-openeular-22.03-lts/localize.sh
buildah run --network=host $container -- bash -c "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -y"
buildah run --network=host $container -- yum install -y NetworkManager-libnm-devel systemd-devel git gcc protobuf-devel
buildah run --network=host $container -- cp /localize-openeular-22.03-lts/config /root/.cargo/config
buildah config --env PATH=/root/.cargo/bin:$PATH $container
buildah run --network=host -v "${projectdir}:/root/workspace" --workingdir /root/workspace $container -- cargo clean
buildah run --network=host -v "${projectdir}:/root/workspace" --workingdir /root/workspace $container -- cargo install --path . --root /usr/local

buildah run --network=host $commit_container -- bash /localize-openeular-22.03-lts/localize.sh
buildah run --network=host $commit_container -- yum install -y NetworkManager-libnm systemd-libs xz tar

buildah add $commit_container https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-noarch.tar.xz /tmp
buildah run --network=host $commit_container -- tar -C / -Jxpf /tmp/s6-overlay-noarch.tar.xz
buildah run --network=host $commit_container -- rm -rf /tmp/s6-overlay-noarch.tar.xz
buildah add $commit_container https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-x86_64.tar.xz /tmp
buildah run --network=host $commit_container -- tar -C / -Jxpf /tmp/s6-overlay-x86_64.tar.xz
buildah run --network=host $commit_container -- rm -rf /tmp/s6-overlay-x86_64.tar.xz
buildah run --network=host $commit_container -- sh -c 'cp -rfv /s6-rc.6/* /etc/s6-overlay/s6-rc.d/'

buildah run --network=host $commit_container -- touch /etc/s6-overlay/s6-rc.d/user/contents.d/orbuculum-web
buildah run --network=host $commit_container -- touch /etc/s6-overlay/s6-rc.d/user/contents.d/orbuculum

buildah run --network=host $commit_container -- yum remove -y xz tar
buildah run --network=host $commit_container -- yum clean all

buildah run --network=host $commit_container -- mkdir /etc/orbuculum
buildah copy $commit_container "${container_mnt}/usr/local/bin/orbuculum"* /usr/bin
buildah run --network=host -v "${projectdir}:/root/workspace" --workingdir /root/workspace $commit_container -- cp assets/examples/rules/nic.rules /etc/orbuculum/
buildah run $commit_container -- sh -c 'rm -rf /s6-rc.6 /localize-*'

buildah config --entrypoint /init --env RUST_LOG=info $commit_container

buildah umount $container
buildah rm $container
buildah commit $commit_container docker.io/ssfdust/orbuculum:v0.0.1-alphav1-openeuler-22.03-lts-amd64
buildah rm $commit_container
