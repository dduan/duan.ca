# Sencha Touch Workflow with GNU Make and Tmux
2013-05-27T15:00:00-06:00
tag: Sencha Touch, GNU Make, CoffeeScript, SASS, Tmux, Bash

I throw this Makefile to the root directory of my Sencha Touch 2 projects for
workflow automation.

Assuming you write in CoffeeScript and run Tmux in a terminal:

`make develop` will put `compass` and `coffee` to watch mode, in addition to
spawning a local web server with Python 3. The three commands will run in three
separate Tmux panes.

`make watch` does the same thing sans the server spawning.

`make` simply compile coffee script files and sass files.

You can figure out the granular commands with some minimal knowledge of GNU
Make.

<script src="https://gist.github.com/DaNmarner/5659003.js"></script>
