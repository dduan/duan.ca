---
title: Swift ABI In Layman's terms
link: https://lists.swift.org/pipermail/swift-evolution/Week-of-Mon-20160808/026153.html
date: 2017-01-16 22:44:38-0800
---

An oldie but goodie from Swift Evelution.
[Slava](https://twitter.com/slava_pestov)'s answer:

> "when we talk about the ABI, we're really talking about three orthogonal
> "axes" along which we would like to "move" without breaking compatibility with
> existing binaries:"
>
> - The first axis is the machine-level calling conventions and memory layout.
>   For example, what registers to pass function arguments and returns in, the
>   rules for alignment and padding of fields in an aggregate type, which entry
>   points the Swift runtime exports and what their behavior should be. Once we
>   commit to a stable ABI along this axis, you will get interoperability between
>   *compiler versions* -- the same exact library built with one version of the
>   compiler will remain compatible with clients after being recompiled with
>   another version, because their conventions will match up. Note that this does
>   not help you if the library itself changes in any way.
>
> - The second axis is the resilience work I called out in my previous e-mail.
>   Here, we're trying to define language features and implementation techniques
>   that allow a library to evolve in a forward-compatible manner, as long as the
>   developer follows certain guidelines. Here, the goal is if you should be able
>   to compile your library, make some changes to add new APIs, and recompile it
>   *with the same compiler*, without breaking downstream clients, as long as you
>   follow the library evolution guidelines (Also, you can imagine one day having
>   an 'ABI diff' tool to automate this).  >
>
> - The third axis is the standard library itself. Stability of runtime
>   interfaces and the extra indirection to enable resilience is all great, but it
>   won't help you as long as the standard library API is evolving in a
>   non-backwards compatible manner -- for example, if we remove a method on
>   String. So once the other two areas have been addressed, the last thing to lock
>   down is the standard library interface itself.

The rest of the thread is worth reading too. John McCall gave an high level
[description](https://lists.swift.org/pipermail/swift-evolution/Week-of-Mon-20160808/026145.html).

The definitive source for Slava's third axes is the [Library
Evolution](https://github.com/apple/swift/blob/master/docs/LibraryEvolution.rst)
doc from the Swift repository.
