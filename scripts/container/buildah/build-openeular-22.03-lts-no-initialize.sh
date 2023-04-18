#!/bin/sh
container=$(buildah from docker.io/ssfdust/orbuculum:v0.0.1-alphav1-openeuler-22.03-lts-amd64)
buildah run $container -- sed -i 's/\(.*orbuculum.*\)/\1 --no-initialize/' /etc/s6-overlay/s6-rc.d/orbuculum/run
buildah commit $container docker.io/ssfdust/orbuculum:v0.0.1-alphav1-openeuler-22.03-lts-no-initialize-amd64
buildah rm $container
