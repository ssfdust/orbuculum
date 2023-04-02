#! /usr/bin/env sh

if command -v docker;then
    docker build . -f Dockerfile-rocky9.dev -t docker.io/ssfdust/orbuculum-dev:latest
elif command -v podman;then
    podman build . -f Dockerfile-rocky9.dev -t docker.io/ssfdust/orbuculum-dev:latest
fi
