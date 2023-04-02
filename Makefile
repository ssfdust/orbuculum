docker:
	cd scripts/container/docker && bash build-dev.sh

docker-rocky9:
	cd scripts/container/docker && bash build-dev-rocky9.sh

buildah:
	cd scripts/container/buildah && bash build-dev.sh
