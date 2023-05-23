#!/bin/sh
BUILD_IMAGE=docker.io/library/debian:bullseye-slim
COMMIT_IMAGE=docker.io/library/debian:bullseye-slim
DISTRIBUTION=bullseye
IMAGE_NAME=docker.io/ssfdust/orbuculum:v0.0.1-alphav1-debian-11-amd64-r2

source common.sh

container=$(buildah from $BUILD_IMAGE)
commit_container=$(buildah from $COMMIT_IMAGE)
container_mnt=$(buildah mount $container)

localize_container $container
buildah run --network=host $container -- apt install -y curl build-essential git libudev-dev libnm-dev libsystemd-dev protobuf-compiler

prepare_rust_env $container
build_package $container

localize_container $commit_container
buildah run --network=host $commit_container -- apt install -y libnm0 xz-utils

add_s6_overlay $commit_container

buildah run --network=host $commit_container -- apt remove -y xz-utils
buildah run --network=host $commit_container -- apt autoremove -y
buildah run --network=host $commit_container -- apt autoclean -y

package_image $commit_container $container_mnt
buildah commit $commit_container $IMAGE_NAME

cleanup $container $commit_container
