#!/bin/sh
ROCKY_VERSION=${1:-8}
BUILD_IMAGE=docker.io/library/rockylinux:$ROCKY_VERSION
COMMIT_IMAGE=docker.io/library/rockylinux:$ROCKY_VERSION-minimal
DISTRIBUTION=rocky$ROCKY_VERSION
IMAGE_NAME=docker.io/ssfdust/orbuculum:v0.0.1-alphav1-rockylinux-$ROCKY_VERSION-amd64-r1

source common.sh

container=$(buildah from $BUILD_IMAGE)
commit_container=$(buildah from $COMMIT_IMAGE)

container_mnt=$(buildah mount $container)

localize_container $container
buildah run --network=host $container -- dnf install -y NetworkManager-libnm-devel systemd-devel git protobuf-devel gcc llvm jq

build_package $container

localize_container $commit_container
buildah run --network=host $commit_container -- microdnf install -y NetworkManager-libnm systemd-libs xz tar

add_s6_overlay $commit_container

buildah run --network=host $commit_container -- microdnf remove -y xz tar
buildah run --network=host $commit_container -- microdnf clean all

package_image $commit_container $container_mnt
buildah commit $commit_container $IMAGE_NAME

cleanup $container $commit_container
