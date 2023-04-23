#!/bin/sh
BUILD_IMAGE=docker.io/library/rockylinux:8
COMMIT_IMAGE=docker.io/library/rockylinux:8-minimal
DISTRIBUTION=rocky8
IMAGE_NAME=docker.io/ssfdust/orbuculum:v0.0.1-alphav1-rockylinux-8-amd64

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
