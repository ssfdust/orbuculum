#!/bin/nu
alias xcargo = podman run --network=host --rm --name test -v $"(pwd | str trim):/work" -v $"($env.HOME)/.cargo:/root/.cargo" --workdir /work --replace -ti docker.io/ssfdust/orbuculum-dev cargo
alias xbash = podman run --network=host --rm --name test -v $"(pwd | str trim):/work" -v $"($env.HOME)/.cargo:/root/.cargo" --workdir /work --replace -ti docker.io/ssfdust/orbuculum-dev
