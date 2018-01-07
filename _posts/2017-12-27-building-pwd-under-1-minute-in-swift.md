---
title: Building 'pwd' Under 1 Minute In Swift
tags: [Swift, Unix, C, YouTube, Minute Swift]
date: 2017-12-27 01:13:07-0800
---

I guess I'm starting a series of minute-long videos. In this first video, I built `pwd` out of `getcwd`. The
code turned out shorter than I imagined (1 line). Which is a good demonstration of Swift's great C-interop
capability.

<div class="video-container">
    <iframe width="560" height="315" src="https://www.youtube.com/embed/YR4MCcMnHrI" frameborder="0" gesture="media" allow="encrypted-media" allowfullscreen></iframe>
</div>

Some details are lost in this video, however. For example, the POSIX standard actually says `getcwd`'s 2nd
argument being `0` is undefined. Luckily, both macOS and Linux define it the way we want (allocate as much
memory as needed). Swift's handling of `char *` aka `UnsafeMutablePointer<CChar>` in context of a `var` array
is also very cool.
