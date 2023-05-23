#!/bin/sh

S6_OVERLAY_VERSION=3.1.4.2
projectdir="$(cd `pwd`;cd ../../..;pwd)"

function build_package() {
    buildah run --network=host -v "${projectdir}:/root/workspace" --workingdir /root/workspace $1 -- cargo clean
    buildah run --network=host -v "${projectdir}:/root/workspace" --workingdir /root/workspace $1 -- cargo install --path . --root /usr/local
}

function add_s6_overlay() {
    buildah add $1 https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-noarch.tar.xz /tmp
    buildah run --network=host $1 -- tar -C / -Jxpf /tmp/s6-overlay-noarch.tar.xz
    buildah run --network=host $1 -- rm -rf /tmp/s6-overlay-noarch.tar.xz
    buildah add $1 https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-x86_64.tar.xz /tmp
    buildah run --network=host $1 -- tar -C / -Jxpf /tmp/s6-overlay-x86_64.tar.xz
    buildah run --network=host $1 -- rm -rf /tmp/s6-overlay-x86_64.tar.xz
    buildah run --network=host $1 -- sh -c 'cp -rfv /s6-rc.d/* /etc/s6-overlay/s6-rc.d/'
}

function prepare_rust_env() {
    buildah run --network=host $1 -- bash -c "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -y"
    buildah run --network=host $1 -- cp /localize-$DISTRIBUTION/config /root/.cargo/config
    buildah config --env PATH=/root/.cargo/bin:$PATH $1
}

function package_image() {
    buildah run --network=host $1 -- mkdir /etc/orbuculum
    buildah copy $1 "$2/usr/local/bin/orbuculum"* /usr/bin
    buildah run --network=host -v "${projectdir}:/root/workspace" --workingdir /root/workspace $1 -- cp assets/examples/rules/prod.rules /etc/orbuculum/default.rules
    buildah run $1 -- sh -c 'rm -rf /s6-rc.d /localize-*'

    buildah config --entrypoint /init $1
}

function localize_container() {
    buildah add $1 assets /
    buildah run --network=host $1 -- bash /localize-$DISTRIBUTION/localize.sh
}

function cleanup () {
    buildah umount $1
    buildah rm $1
    buildah rm $2
}
