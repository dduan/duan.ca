DEPLOY_PATH=/tmp/jekyll_deploy

serve:
	bundle exec jekyll serve --draft --host=0.0.0.0

build:
	bundle exec jekyll build

deploy:
	git checkout -f gh-pages
	git clean -d -x -f
	git pull
	git checkout master
	bundle exec jekyll build
	rm -rf ${DEPLOY_PATH}
	mkdir ${DEPLOY_PATH}
	cp -R .git ${DEPLOY_PATH}
	cd ${DEPLOY_PATH}; git checkout gh-pages; git clean -d -x -f
	cp -R _site/* ${DEPLOY_PATH}
	cd ${DEPLOY_PATH}; git add .; git commit -m "`curl whatthecommit.com/index.txt`"
	cd ${DEPLOY_PATH}; git push -f origin gh-pages
