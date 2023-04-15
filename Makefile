docker:
	cd scripts/container/docker && bash build-dev.sh

docker-rocky9:
	cd scripts/container/docker && bash build-dev-rocky9.sh

buildah:
	cd scripts/container/buildah && bash build-dev.sh

deploy-rocky8:
	cd scripts/container/buildah && buildah unshare bash build-rockylinux-8.sh
	cd scripts/container/buildah && bash build-rockylinux-8-no-initialize.sh

deploy-rocky9:
	cd scripts/container/buildah && buildah unshare bash build-rockylinux-9.sh
	cd scripts/container/buildah && bash build-rockylinux-9-no-initialize.sh

podman-run-rocky9:
	podman run --name orbuculum -d --network=host --privileged -p 15051:15051 -p 9094:9094 -v /dev:/dev -v /sys:/sys -v /var/run/dbus/system_bus_socket:/var/run/dbus/system_bus_socket docker.io/ssfdust/orbuculum:v0.0.1-rockylinux-9-no-initialize-amd64
