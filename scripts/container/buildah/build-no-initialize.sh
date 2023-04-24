#!/bin/sh
DISTRIBUTIONS=("debian-11" "rockylinux-8" "rockylinux-9")
for distribution in ${DISTRIBUTIONS[@]}
do
    container=$(buildah from docker.io/ssfdust/orbuculum:v0.0.1-alphav1-${distribution}-amd64-r1)
    buildah run $container -- sed -i 's/\(.*orbuculum.*\)/\1 --no-initialize/' /etc/s6-overlay/s6-rc.d/orbuculum/run
    buildah commit $container docker.io/ssfdust/orbuculum:v0.0.1-alphav1-${distribution}-no-initialize-amd64-r1
    buildah rm $container
done
