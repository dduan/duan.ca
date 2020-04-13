---
title: Git Commit Message, Vim, and Markdown
tags: [Git, Vim, Markdown, GitHub]
date: 2020-04-13T13:41:02-07:00
---

It's been bothering me for years.

That is, `#` is both the start of a comment in a git commit message, but also
the syntax for headings in Markdown.

Personally, I prefer using commit messages to populate pull request descriptions
whenever possible. On GitHub, This happens automatically when the pull request
contains a single commit. But I can't type, say an H3 as `### Heading` in the
commit message (in Vim, most of the time) because it gets treated as a comment!

But thanks to this twitter interaction with [@dmartincy][], I finally solved
this problem:

<blockquote class="twitter-tweet"><p lang="en" dir="ltr">I haven&#39;t found myself in that situation, but maybe you could do &#39;git config core.commentChar &quot;;&quot;&#39; before creating the commits? That will change the default git commit marker (#) to something that won&#39;t conflict with Markdown titles.</p>&mdash; Daniel Martín (@dmartincy) <a href="https://twitter.com/dmartincy/status/1247271508420026368?ref_src=twsrc%5Etfw">April 6, 2020</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>


As mentioned by Daniel, Git has introduced a setting called `core.commentChar`,
documented [here][core.commentChar], which lets us control which character
becomes the start of a comment line. Let's say we want to replace the default
`#` with `;`, we can edit `~/.gitconfig` to include this preference:

```
[core]
	commentChar = ";"
```

… and this will affect every Git repository on this computer.

For me, though, this broke git commit syntax highlighting in Vim: the comments,
beginning with `;`, are no longer recognized as comments. To fix this, I updated
Vim's syntax for file type `gitcommit`. In your Vim setting directory
(`~/.config/nvim/` for me), create a file `syntax/gitcommit.vim` file (unless
you already have one), add the following line:

```
syn match gitcommitComment	"^;.*"
```

Note `;` matches my preferred `core.commentChar` for Git.

Et voilà! Git commit message looks tippy-top in Vim again!

![editing git commit in vim, with alternative character being the beginning of a comment](/assets/2020/04/gitcommit.png)

[@dmartincy]: https://twitter.com/dmartincy
[core.commentChar]: https://git-scm.com/docs/git-config#Documentation/git-config.txt-corecommentChar
