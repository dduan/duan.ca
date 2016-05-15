DEPLOY_PATH=/tmp/jekyll_deploy
build:
	jekyll build

deploy:
	git checkout -f gh-pages
	git clean -d -x -f
	git pull
	git checkout master
	jekyll build
	rm -rf ${DEPLOY_PATH}
	mkdir ${DEPLOY_PATH}
	cp -R .git ${DEPLOY_PATH}
	cd ${DEPLOY_PATH}; git checkout gh-pages; git clean -d -x -f
	cp -R _site/* ${DEPLOY_PATH}
	cd ${DEPLOY_PATH}; git add .; git commit -m "`curl whatthecommit.com/index.txt`"
	cd ${DEPLOY_PATH}; git push -f origin gh-pages
