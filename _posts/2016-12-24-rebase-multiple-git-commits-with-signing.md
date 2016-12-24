---
title: Rebase Multiple Git Commits With Signing
tags: [Git]
date: 2016-12-24 01:31:14-0800
---

I never bothered [signing][signing] my git commits until Github started putting
those shiny badges on them. Since then I've encountered a problem: rebased
commits lose their hash and their signed-ness.

That's not a big deal for a single commit. Just run `git commit -S --amend
--no-edit` after rebase. And that commit, which is at `HEAD` will have a new
hash and be signed.

What if we have more than one commit to rebase? My instinctive answer was sign
the one on `HEAD`, then use interactive rebase to rotate the next one on top and
sign it, repeat until each one is signed and reorder them. But that doesn't work
at all because a commit's position in history is part of its content. That means
after a commit is rebased, it and every commits following it will have a new
hash and git doesn't give us an oppurtunity to say "include my signature,
please". You cannot rebase and keep the commits signed!

Well, that kind of sucks. But you are reading about it here, so I'd better give
you something to help, right?

There is, at least, one way to achieve *rebase and sign multiple commits*:

1. Run interactive rebase: `git rebase -i branch`.
2. Mark all commits and "edit" or "e".
3. Git now stops at every commit for you to make changes. This is an oppurtunity
   to sign it! `git commit -S --amend --no-edit`, just like how you would sign
   `HEAD`.
4. `git rebase --continue`, move on the next commit until all every one is
   edited.


There's virtually nothing you cannot do with git. Hence the reputation for it's
glorious UI, ya know?

[signing]: https://git-scm.com/book/en/v2/Git-Tools-Signing-Your-Work
