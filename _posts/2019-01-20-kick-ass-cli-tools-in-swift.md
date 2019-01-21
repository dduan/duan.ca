---
title: Kick-ass CLI Tools In Swift
tags: [Swift, CLI, POSIX]
date: 2019-01-20 16:09:38-0800
---

As someone who lives in a terminal simulator, I'm pleasantly surprised by the
new toys we get in recent years such as [fzf][], [ripgrep][], [fd][], etc.
A great number of these are written in relatively young programming languages
such as Go and Rust. But, noticibly, none of them are written in Swift.

In this post, I'll try to explain why that is.

## POSIX Ergonomics

Unix-like virtual file systems has been around for decades. API that manupulates
such systems has standardized a long time ago and exists in most computers
running Linux/BSD/macOS today (and, to a large extend, smart phones). To Swift
users, Using these APIs is straight-forward (`rmdir("path/to/dir")`).

So Swift programmers are all happy campers (re-)inventeing all sorts of file
system utilities, right?

Well, not quite.

Okay, I lied about POSIX APIs being "straight-forward" in Swift. Or rather, this
is very subjective.

Continuing with the `rmdir` example, we must first import it from either `Glibc`
or `Darwin`, depending on your OS. To know whether the operation is successful,
we need to see whether it returned integer 0. To learn _why_ 0 was not returned,
we need to read the "magical" variable `errno`. `errno` could be written to by
other APIs so we'd better capture it in time…

And that's one of the simpler APIs in POSIX calls!

Programmers whine about ergonomics partially because we are previlidged and
spoiled. But mostly because our attention is a limited resources. Mixing API
conventions distracts us from solving the problem at hand. Bad ergonomics,
therefore, drives away a good potion of users who cares about quality of their
tools.

## Culture and History

As of this writing, the release of Swift 5 is imminent. The vast majority of
existing Swift code is written to run on iOS. The concept of a file, or the
traditional virtal file system, is hidden to iOS users, and sandboxed for
developers. I bet most Swift users rarely think about the fact that there's
a entire set of POSIX API at their disposal.

`Foundation` alleviates the need to deal with files and directories: `Bundle`
locates the files; `CoreData`, `UserDefaults` or the keychain is your primary
way to persist data; `Data`, `String` or `NSCoding` has methods to read and
write to files.  And finally, if you really need to deal with files,
`NSFileManager` has everything you'll ever need.

Why would a productive Swift programmer think about POSIX in this environment?
Why would a tutor teach POSIX over the useful/practical/"native" alternatives?

We can trace "riding on the Apple platform" mentality back to the pre-iPhone
days, where a very small Mac developer community labors on on a niche platform
(compared to iOS today) and they _loved_ it. However, I'm sure they used more
POSIX stuff back then than the average iOS developers today.

Having a great library such as Foundation on the most popular developer
platform where the language thrives means it'll take longer for "subcultures"
to emerge, if they do at all.

## The Standard Library And Its Influence on New Users

File system APIs being in `Foundation` as opposed to the standard library is
probably a temporary condition. Nevertheless, it has at least the following
implications:

1. Its quality of implementation is not held on the same standard that those
   APIs in the standard library. This is especially true for the separate,
   open-source `Foundation` implementation. Getting consistent and correct
   behaviors across macOS and Linux is hard.

2. A person learning Swift won't explore the language with a file system API.
   This I suspect, is __the most important reason many of these great CLI
   utilites are written in other programming languages__. Programmers seek
   instant gratification when they learn. And they usually stay in a limited
   domain (like iOS) at first. This is where the built-in library is special: no
   matter which domain is chosen, it's always available. Languages such as Go
   and Rust include things like paths and files in their built-in library.
   Playing with these APIs while learning the lanugage plants a seed for future,
   serious, projects. There are less users of these languages compared to Swift,
   but there are more people thinking about projects that involves file systems
   in thoes communities. (Note I don't have statistics here, just a guess.)

## Conclusion

The next killer CLI tool is still more likely to be written in Go or Rust,
than in Swift. Hopefully, somewhere in these speculations is a true cause of
this phenomena. Maybe someone reading this will be inspired to accelerate change
that will eventually revert the condition. (I'm [trying][pathos]).

[fzf]: https://github.com/junegunn/fzf
[fd]: https://github.com/sharkdp/fd
[ripgrep]: https://github.com/BurntSushi/ripgrep
[pathos]: https://github.com/dduan/Pathos
