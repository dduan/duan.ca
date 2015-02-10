+++
date = "2012-11-29T09:37:30-06:00"
title = "MongoDB, PHP And Mountain Lion"
tags = [ "PHP", "MongoDB", "Mac", "OS X Mountain Lion" ]
slug = "MongoDB-PHP-And-Mountain-Lion"
+++

Here are the necessary steps to make PHP work with MongoDB on vanilla
Mac OS X 10.8 (Mountain Lion):

1.  Install [Command Lion Tools for Xcode][1]
2.  Install Homebrew:

        ruby -e "$(curl -fsSkL raw.github.com/mxcl/homebrew/go)"

3.  Prioritize Homebrew's binaries in $PATH:

        echo "export /usr/local/bin:$PATH" >> ~/.bash_profile

4.  Install PHP 5.4 with Homebrew:

        # Setup the tap for dependencies
        brew tap homebrew/dupes
        # This is the best 
        brew tap josegonzalez/homebrew-php
        # And install
        brew install php54

5.  Install MongoDB with Homebrew

        brew install mongodb

6.  Install "PHP Driver" for MongoDB:

        sudo pecl install mongo

7.  Done.

Now go run some PHP-MongoDB scripts with the build-in web server from
PHP 5.4 like you normally would (don't forget to start `mongod`) and everything
should work.

[1]: https://developer.apple.com/downloads/index.action
