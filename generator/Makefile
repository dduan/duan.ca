OS:=$(shell uname -s)
image=duan.ca:1.0

build:
	@cargo build --release
	@cp target/release/generator ../bin/generate-$(OS)

test:
	@cargo test

build-docker:
ifeq ($(shell docker image inspect $(image) 2>/dev/null),[])
	@docker build --tag $(image) .
endif
	@docker run --rm \
	    --user "$(id -u)":"$(id -g)" \
	    -v "$(PWD)":/usr/src/duan.ca \
	    -w /usr/src/duan.ca \
	    $(image) \
	    /bin/sh -c "cargo build --release --target x86_64-unknown-linux-musl"
	@cp target/x86_64-unknown-linux-musl/release/generator ../bin/generate-Linux
