build:
	@zola build
	@scripts/fix-feed-name.sh

serve: build
	@zola serve -i 0.0.0.0 -u $(shell ipconfig getifaddr en0) -p 8000
