OS:=$(shell uname -s)
ARCH:=$(shell uname -m)
# Linux architecture to target for static (musl) builds. Defaults to the host's,
# with macOS's "arm64" spelled the Linux way. Override to cross-build, e.g.
# `make build-docker LINUX_ARCH=x86_64`.
LINUX_ARCH?=$(subst arm64,aarch64,$(ARCH))
DOCKER_PLATFORM:=linux/$(subst x86_64,amd64,$(subst aarch64,arm64,$(LINUX_ARCH)))
MUSL_TARGET:=$(LINUX_ARCH)-unknown-linux-musl
image=duan.ca:1.0-$(LINUX_ARCH)

# Native build for the host, e.g. bin/generate-Darwin-arm64.
build:
	@cargo build --release
	@cp target/release/generator ../bin/generate-$(OS)-$(ARCH)

test:
	@cargo test

# Static Linux build for the host architecture, using a local rustup toolchain
# (`rustup target add $(MUSL_TARGET)` first).
build-static:
	@cargo build --release --target $(MUSL_TARGET)
	@cp target/$(MUSL_TARGET)/release/generator ../bin/generate-Linux-$(LINUX_ARCH)

# Static Linux build in Docker, for LINUX_ARCH (host architecture by default).
build-docker:
ifeq ($(shell docker image inspect $(image) 2>/dev/null),[])
	@docker build --platform $(DOCKER_PLATFORM) --tag $(image) .
endif
	@docker run --rm \
	    --platform $(DOCKER_PLATFORM) \
	    --user "$(shell id -u)":"$(shell id -g)" \
	    -v "$(PWD)":/usr/src/duan.ca \
	    -w /usr/src/duan.ca \
	    $(image) \
	    /bin/sh -c "cargo build --release --target $(MUSL_TARGET)"
	@cp target/$(MUSL_TARGET)/release/generator ../bin/generate-Linux-$(LINUX_ARCH)
