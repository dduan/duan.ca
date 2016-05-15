DEPLOY_PATH=/tmp/jekyll_deploy
build:
	jekyll build

deploy: build
	git checkout -f gh-pages
	git clean -d -x -f
	git pull
	git checkout master
	rm -rf ${DEPLOY_PATH}
	mkdir ${DEPLOY_PATH}
	cp -R .git ${DEPLOY_PATH}
	cd ${DEPLOY_PATH}; git checkout gh-pages; git clean -d -x -f
	cp -R _site/* ${DEPLOY_PATH}
	cd ${DEPLOY_PATH}; git commit -am "`curl whatthecommit.com/index.txt`"
