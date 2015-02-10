+++
date = "2012-10-31T10:02:30-06:00"
title = "Keep Git Social"
tags = [ "Git" ]
slug = "Keep-Git-Social"
+++

*A project I'm working on uses [GitlabHQ][4], I think it is a cool open source
project and the developers are doing an excellent job of making a sweet web
interface for Git projects. However, as I found out today, [my one issue][1]
with it was closed without fixing. And that's what finally propelled me to
writing this airticle.*

Do you know, the Linux kernel is broken by hundreds of commited patches
everyday?

That, of course, is a trick question. Yes, *technically*, hundreds (if not
thousands) of patches were commited to *a* git repository of the Linux kernel
that isn't different in anyway from *the* repository. What makes the
repository "conanical" is the fact that Linus Torvalds owns it. If the world
lost access to his hard drive because Linus was hit by a bus (I would never
wish this to happen) today, all the Linux community need to do is figure out
who is next BDFL. Then his/hers would become *the* Linux repository.

In other words, what makes the difference is purely social.

**Not trying to fit the social structure of a software project into a
repository is the biggest strength of git.** This is why I frown everytime
I see a discussion about locking down branches of a Git repository.

Making developers "own" branches on a shared repository is a pratice
inheritated from the days of centralized version control. Git doesn't provide
built-in support for this, for good reasons.

With Subversion, the ultimate goal of branch-wise access control is to keep out
bad changes made by developers while give them the benefits of version
control, namely:

1.  provide a backup of the change history
2.  let others to follow his/her progress so that they can contribute via
    patches or collaborate if they have write access.

With Git, these are easily achieved by a personal repository read-accessible
for the team. 

Meanwhile, write access is granted at the repository level, but only to very
few or, more preferably, one person. This is possible because each team member
can have and only push to his/her own repositories. No change made by others 
will make their way to the "conanical" repository unless its owner(s) activaly
pulls them in. Bad changes therefore is kept out.

Additionally, Git has [features][2] flexible enough to
support all kinds of [development process][3]es, partially because
its branches, by design, are not responsible for enforcing access
permissions. 

Consider developer John working on a shared git repository with
locked branches: where would he push his temporary branch to for backup,
if he can only write to `feature42` and `johns_branch`? How does he get
`emergency_bugfix_14159` pair reviewed at 4am? If he uses his own repo
for those, does it mean he has to force the whole team to know about it?
How does it affect work when a `git branch --all` produces a phone book?

Break Git's design gets you no where nice.

And no, Git's social model doesn't add work to the owner of the "official"
repository. Afterall, Git was designed with he linux kernel in mind!
Linus himself explains it the best (summary provided below):

<iframe width="420" height="315"
src="http://www.youtube.com/embed/4XpnKHJAok8"
frameborder="0" allowfullscreen></iframe>

In short, Linus only review pull requests from a few "lieutenants" he
trusts, and they each follow the same process with their trusted few.
And the pyramid trickles all the way down. Here agian, Git solves a
problem by getting out of the way of the project's social stucture,
instead of trying to encapsulate it.

Git was the first version control system I've ever used starting in 2008. 
I've since gradually realized that not everyone is lucky like me, in the
sense that svn has been working just fine for a lot of people. When their
project switches to Git, emulating the svn/old workflow with functionality
provided by projects like `gitolite` is only natural. But if you are one of
them, and want more creativity from your team, perhaps embracing the
social aspect of Git by breaking the shackles on their hand is a good thing
to try.


[1]: https://github.com/gitlabhq/gitlabhq/issues/1298 "Gitlab Issue: Closed"
[2]: http://git-scm.com/book/en/Git-Branching
[3]: http://nvie.com/posts/a-successful-git-branching-model
[4]: http://gitlabhq.com

