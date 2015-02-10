+++
date = "2012-11-28T15:11:00-06:00"
title = "CoffeeScript And Mountain Lion"
tags = [ "Node.js", "CoffeeScript", "Mac", "OS X Mountain Lion" ]
slug = "CoffeeScript-And-Mountain-Lion"
+++

Here are the necessary steps to install CoffeeScript on
Mac OS X 10.8 (Mountain Lion):

1.  Install [Command Lion Tools for Xcode][1]
2.  Install Homebrew:

        ruby -e "$(curl -fsSkL raw.github.com/mxcl/homebrew/go)"

3.  Install Node.js with Homebrew:

        brew install node

4.  Install npm (the version comes with node doesn't install CoffeeScript
    properly for some reason):

        curl https://npmjs.org/install.sh | sh

5.  Install CoffeeScript with npm:

        npm install -g coffee-script
        # add environment variable to ~/.bashrc
        echo "export NODE_PATH=/usr/local/lib/node_modules"
        . ~/.bashrc

6.  Done!

I had to google around to figure out the environment variable and the issue
with npm. Hopefully this will help some folks in the similar need.

[1]: https://developer.apple.com/downloads/index.action
