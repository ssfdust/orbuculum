#! /usr/bin/env sh
# proxy="-e HTTPS_PROXY=http://192.168.1.81:10801 -e HTTP_PROXY=http://192.168.1.81:10801 -e ALL_PROXY=http://192.168.1.81:10801"
alias xcargo="docker run --network=host --rm --name test $proxy -v '$(pwd):/work' -v '$HOME/.cargo/git:/root/.cargo/git:rw' -v '$HOME/.cargo/registry:/root/.cargo/registry:rw' --workdir /work -ti docker.io/ssfdust/orbuculum-dev cargo"
alias xbash="docker run --network=host --rm --name test $proxy -v '$(pwd):/work' -v '$HOME/.cargo/git:/root/.cargo/git:rw' -v '$HOME/.cargo/registry:/root/.cargo/registry:rw' --workdir /work -ti docker.io/ssfdust/orbuculum-dev bash"
