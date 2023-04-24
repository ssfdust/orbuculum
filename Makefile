docker:
	cd scripts/container/docker && bash build-dev.sh

docker-rocky9:
	cd scripts/container/docker && bash build-dev-rocky9.sh

buildah:
	cd scripts/container/buildah && bash build-dev.sh

deploy-debian-11:
	cd scripts/container/buildah && buildah unshare bash -x build-debian-11.sh

deploy-rocky8:
	cd scripts/container/buildah && buildah unshare bash -x build-rockylinux-8.sh

deploy-rocky9:
	cd scripts/container/buildah && buildah unshare bash -x build-rockylinux-9.sh

deploy-no-initialize:
	cd scripts/container/buildah && buildah unshare bash -x build-no-initialize.sh

upload-images:
	podman push docker.io/ssfdust/orbuculum:v0.0.1-alphav1-debian-11-amd64-r1
	podman push docker.io/ssfdust/orbuculum:v0.0.1-alphav1-rockylinux-8-amd64-r1
	podman push docker.io/ssfdust/orbuculum:v0.0.1-alphav1-rockylinux-9-amd64-r1
	podman push docker.io/ssfdust/orbuculum:v0.0.1-alphav1-debian-11-no-initialize-amd64-r1
	podman push docker.io/ssfdust/orbuculum:v0.0.1-alphav1-rockylinux-8-no-initialize-amd64-r1
	podman push docker.io/ssfdust/orbuculum:v0.0.1-alphav1-rockylinux-9-no-initialize-amd64-r1
