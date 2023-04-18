docker:
	cd scripts/container/docker && bash build-dev.sh

docker-rocky9:
	cd scripts/container/docker && bash build-dev-rocky9.sh

buildah:
	cd scripts/container/buildah && bash build-dev.sh

deploy-openeuler:
	cd scripts/container/buildah && buildah unshare bash -x build-openeular-22.03-lts.sh
	# cd scripts/container/buildah && bash -x build-centos-7-no-initialize.sh

deploy-rocky8:
	cd scripts/container/buildah && buildah unshare bash -x build-rockylinux-8.sh
	cd scripts/container/buildah && bash -x build-rockylinux-8-no-initialize.sh

deploy-rocky9:
	cd scripts/container/buildah && buildah unshare bash -x build-rockylinux-9.sh
	cd scripts/container/buildah && bash -x build-rockylinux-9-no-initialize.sh

podman-run-rocky9:
	podman run --name orbuculum -d --network=host --privileged -v /run/udev:/run/udev -v /var/run/dbus/system_bus_socket:/var/run/dbus/system_bus_socket docker.io/ssfdust/orbuculum:v0.0.1-rockylinux-9-no-initialize-amd64
