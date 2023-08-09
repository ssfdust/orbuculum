#!/bin/sh
BUILD_IMAGE=docker.io/library/debian:buster
COMMIT_IMAGE=docker.io/library/debian:buster
IMAGE_NAME=docker.io/ssfdust/orbuculum-old-dev:buster
DISTRIBUTION=bullseye

source common.sh

container=$(buildah from $BUILD_IMAGE)

localize_container $container
buildah run --network=host $container -- apt install -y curl build-essential git libudev-dev libnm-dev libsystemd-dev protobuf-compiler
prepare_rust_env $container

buildah commit $container $IMAGE_NAME
buildah rm $container
