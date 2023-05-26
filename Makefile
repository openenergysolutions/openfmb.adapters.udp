PLATFORMS ?= linux/amd64,linux/arm64
VERSION ?= switch-profile
IMAGE_NAME ?= oesinc/openfmb.adapters.udp

ifneq ($(TARGET),)
	TARGET_OPT=--target=$(TARGET)
endif

ifeq ($(BUILD_TYPE),Debug)
	BUILD_DIR=target/${TARGET}/debug
	CARGOFLAGS=
else 
	BUILD_DIR=target/${TARGET}/release
	CARGOFLAGS=--release
endif

.PHONY: build clean docker

build:
	cargo build ${CARGOFLAGS} ${TARGET_OPT}

clean:
	rm -rf target

docker:
	docker buildx build \
		--platform $(PLATFORMS) \
		-t $(IMAGE_NAME):$(VERSION) \
		--push \
		.
