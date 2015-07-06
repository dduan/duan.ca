+++
date = "2015-07-05T22:36:09.242959-07:00"
title = "Dynamic Swift Framework Without Xcode"
slug = "dynamic-swift-framework-without-xcode"
tags = [ "Swift", "Xcode" ]
+++

I came up with this question recently:

<blockquote class="twitter-tweet" lang="en"><p lang="en" dir="ltr">Can we even use Frameworks with <a href="https://twitter.com/hashtag/Swiftlang?src=hash">#Swiftlang</a> on Linux?</p>&mdash; Daniel Duan (@daniel_duan) <a href="https://twitter.com/daniel_duan/status/617470929241706496">July 4, 2015</a></blockquote> <script async src="//platform.twitter.com/widgets.js" charset="utf-8"></script>

And I'm going to give the answer in this post (spoiler alert: yes, sort of).

Here's the content of a framework created by Xcode:

![Just Framework Structure](images/2015/07/just-framework-structure.png)

Some of the files, such as `Info.plist`, are obviously construct of Xcode.
Play with `swiftc` long enough, one would find that the `.swiftdoc`s, the
`.swiftmodule`s and the one binary file came from the Swift compiler.

Instead of listing the relevant `swiftc` command options, I've created a [sample project](https://github.com/dduan/Swift-Framework-Without-Xcode) to demonstrate how one can complie and link to frameworks so that they can be `import`ed in the application code. The key ingredient for achieving it lies in *Makefile*. In summary, `swiftc` can do these for us:

1. generate a binary as a library/framework
2. emit a `.swiftmodule` file, which Swift needs to understand that binary.
3. assign a path the Swift runtime needs to locate this framework.
4. compile source code that imports the framework, given that they exist in the paths relative to the app binary in step 3.


Based on these observations, it's not hard to imagine more sophisticated build systems, such as IDEs and package/dependency management systems.
