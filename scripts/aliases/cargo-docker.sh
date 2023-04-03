#! /usr/bin/env sh
# proxy="-e HTTPS_PROXY=http://192.168.1.81:10801 -e HTTP_PROXY=http://192.168.1.81:10801 -e ALL_PROXY=http://192.168.1.81:10801"

rust_log="-e RUST_LOG=debug"

alias xcargo="docker run --network=host --rm $proxy $rust_log -v '$(pwd):/work' -v '$HOME/.cargo/git:/root/.cargo/git:rw' -v '$HOME/.cargo/registry:/root/.cargo/registry:rw' --workdir /work -ti docker.io/ssfdust/orbuculum-dev cargo"
alias xbash="docker run --network=host --rm $proxy -v '$(pwd):/work' -v '$HOME/.cargo/git:/root/.cargo/git:rw' -v '$HOME/.cargo/registry:/root/.cargo/registry:rw' --workdir /work -ti docker.io/ssfdust/orbuculum-dev bash"
alias xstop="docker ps -a | grep orbuculum | awk '{ print \$1 }' | xargs docker stop"
