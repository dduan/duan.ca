OS:=$(shell uname -s)
build:
	@bin/generate-$(OS) "https://duan.ca" ./ ./public

build-local:
	@bin/generate-$(OS) "http://localhost:8000" ./ ./public

serve: build-local
	@cd public; python3 -m http.server
