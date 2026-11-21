OS:=$(shell uname -s)
ARCH:=$(shell uname -m)
GENERATOR:=bin/generate-$(OS)-$(ARCH)

build:
	@$(GENERATOR) "https://duan.ca" ./ ./public

build-local:
	@$(GENERATOR) "http://localhost:8000" ./ ./public

serve: build-local
	@cd public; python3 -m http.server
