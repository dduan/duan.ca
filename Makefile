build:
	@zola build
	@scripts/fix-feed-name.sh
