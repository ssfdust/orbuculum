docker:
	cd scripts/container/docker && bash build-dev.sh

docker-rocky9:
	cd scripts/container/docker && bash build-dev-rocky9.sh

buildah:
	cd scripts/container/buildah && bash build-dev.sh

deploy-debian-11:
	cd scripts/container/buildah && buildah unshare bash -x build-bullseye.sh

deploy-rocky8:
	cd scripts/container/buildah && buildah unshare bash -x build-rockylinux.sh 8

deploy-rocky9:
	cd scripts/container/buildah && buildah unshare bash -x build-rockylinux.sh 9

deploy-no-initialize:
	cd scripts/container/buildah && buildah unshare bash -x build-no-initialize.sh

upload-images:
	podman push docker.io/ssfdust/orbuculum:v0.0.1-alphav2-debian-11-amd64-r1
	podman push docker.io/ssfdust/orbuculum:v0.0.1-alphav2-rockylinux-8-amd64-r1
	podman push docker.io/ssfdust/orbuculum:v0.0.1-alphav2-rockylinux-9-amd64-r1
	podman push docker.io/ssfdust/orbuculum:v0.0.1-alphav2-debian-11-no-initialize-amd64-r1
	podman push docker.io/ssfdust/orbuculum:v0.0.1-alphav2-rockylinux-8-no-initialize-amd64-r1
	podman push docker.io/ssfdust/orbuculum:v0.0.1-alphav2-rockylinux-9-no-initialize-amd64-r1
