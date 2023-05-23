#!/bin/sh
ROCKY_VERSION=${1:-8}
BUILD_IMAGE=docker.io/library/rockylinux:$ROCKY_VERSION
IMAGE_NAME=docker.io/ssfdust/dashboard-jp
DISTRIBUTION=rocky$ROCKY_VERSION

source common.sh

container=$(buildah from $BUILD_IMAGE)

localize_container $container
buildah run --network=host $container -- dnf install -y git gcc
prepare_rust_env $container

buildah commit $container $IMAGE_NAME
buildah rm $container
