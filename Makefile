EMVER := $(shell yq e ".version" manifest.yaml)
VERSION := 'v0.1'
ASSET_PATHS := $(shell find ./assets/*)
MVCTEST_SRC := $(shell find ./SampleMvcApp)
CONFIGURATOR_SRC := $(shell find ./configurator/src) configurator/Cargo.toml configurator/Cargo.lock
S9PK_PATH=$(shell find . -name mvctest.s9pk -print)

.DELETE_ON_ERROR:

all: verify

verify: mvctest.s9pk $(S9PK_PATH)
	embassy-sdk verify s9pk $(S9PK_PATH)

mvctest.s9pk: manifest.yaml LICENSE image.tar icon.png $(ASSET_PATHS)
	embassy-sdk pack

image.tar: docker_entrypoint.sh configurator/target/aarch64-unknown-linux-musl/release/configurator $(MVCTEST_SRC) $(ASSET_PATHS) Dockerfile
	DOCKER_CLI_EXPERIMENTAL=enabled docker buildx build --platform=linux/arm64/v8  --tag start9/mvctest/main:${EMVER} -o type=docker,dest=image.tar -f ./Dockerfile . 

configurator/target/aarch64-unknown-linux-musl/release/configurator: $(CONFIGURATOR_SRC)
	docker run --rm -it -v ~/.cargo/registry:/root/.cargo/registry -v "$(shell pwd)"/configurator:/home/rust/src start9/rust-musl-cross:aarch64-musl cargo +beta build --release

clean:
	rm image.tar