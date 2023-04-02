BUILDER_IMG_TAG:=20230331
BUILDER_IMG:=ghcr.io/enchant97/trunk-docker-builder:$(BUILDER_IMG_TAG)

.PHONY: help build-in-docker

help:
	@echo "Usage: make [target]"
	@echo "  help"
	@echo "  build-in-docker - Build the app in docker"

build-in-docker:
	@docker run --rm -v $(shell pwd):/app $(BUILDER_IMG)

gh-build-docker:
	@docker run --rm --user root -v $(shell pwd):/app $(BUILDER_IMG)
	@sudo bash ./scripts/gzip_static.sh ./dist
