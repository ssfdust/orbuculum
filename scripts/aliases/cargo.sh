alias xcargo='podman run --rm --name build-orbuculum -v "$(pwd):/work" --workdir /work --replace -ti docker.io/ssfdust/orbuculum-dev:rocky8 cargo'
