---
title: A Case For OOP?
tags: [Python, OOP, Swift]
date: 2016-05-19 12:03:02-0700
---

Python's standard library includes a very handy `defaultdict`. It behaves
almost exactly like the standard dictionary except it'll supply a pre-defined
value for any non-existence keys. It is, unsurpringly, a subclass of `dict`.

I find my self missing this handy container in Swift. Especially when I use
a normal `Dictionary` to accumulate/coalesce values under distinct keys. So I
wrote my own:

<script src="https://gist.github.com/dduan/31ed39c4c98ecb88290f0743cb394c20.js"></script>

There are a few noticable things about this implementation:

* It does not conform to the `DictionaryLiteralConvertible` protocol, for no
  good reasons, really. The initializer in this protocol takes a varadic
  argument. There's no conevient way to forward this array to a normal
  dictionary's initializer (incidentally, this is a Swift feature I really
  want). Plus, I don't need `DefaultDictionary` to be a literal convertible.
* Most of the code, including the imaginary `init` mentioned in previous
  point, simply reuses stuff from `Dictionary`: asscociated type, indexes,
  generator, subscript, etc.

In comparison, Python implements `defaultdict` in a more intuitive way -- via
inheritance.

But do we want inheritance for `struct`s and `enum`s in Swift? What does that
even mean? Is it simply a case that will go away when protocols become more
powerful?
