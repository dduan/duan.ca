---
title: How To Omit Needless Words
tags: [Swift, API Design Guidelines, Naming]
date: 2016-10-22 17:08:41-0700
---

A typical experience in adopting Swift 3 may look like this:

* Run the migrator.
* Fix up what the migrator left undone so your project complies.
* Remove artifacts that migrator added to ensure successful compilation (you
  know what this means if you finished previous step in a mildly sized code
  base).
* Fix bugs caused by mechanical changes.
* Try to adapt your code base to the new style: lowercased static members, etc.

The last step is particularly challenging in a team environment because it
involves naming things. Luckily we have (IMHO) the best "features" in Swift
3 for help: the [API Design Guidelines][Guidelines].

***

[Omit needless words][Omit needless words] is one of the most differentiating
guideline for function names between Swift 2 and 3. In Swift 2 we used to name
things like so…

```
func doStuffWithThing(thing: Thing) { … }
```

… and use it like …

```
doStuffWithThing(someThing)
```

That's a lot of "thing"s, have you noticed?

Since the word "Thing" is *merely repeating* the type information, we can omit
it in Swift 3:

```
// declare
func doStuff(with thing: Thing) { … }

// use
doStuff(with: someThing)
```

It's clear at both declaration and use site what we are doing stuff to.

So … you should go through you code base and make this change! In the next
section, we'll discuss one way to make it happen.

[Guidelines]: https://swift.org/documentation/api-design-guidelines/
[Omit needless words]: https://swift.org/documentation/api-design-guidelines/#omit-needless-words

***

## Step 1: Automate ##

A good indicator of "merely repeating type information" is repetition of words.
If you have been disciplined about naming your functions in Swift 2, finding
such repetition should take no more than one or two good regular expressions.
After the migrator, your code may contain a mixture of these:

```swift
func doStuffWithX(_: X) { … }
func doStuff(forY y: Y) { … }
```

In other words, the repeating word (`X` or `Y`) is preceded by a
[preposition][prepositions].

Finding these in a large code base is quite fun. If you don't feel like writing
the scripts yourself, I've made a little tool for it [here][needless].

(Sometimes the repeating words aren't a exact match because of prefixes such as
"CG" in `CGSize`. That's covered by the tool as well.)


[prepositions]: https://www.englishclub.com/grammar/prepositions-list.htm
[needless]: https://github.com/dduan/needless

## Step 2: Update ##

You can further rearrange the pieces in function name mechanically to:

```swift
func doStuff(preposition originalArgName: Type) { … }
```

The [tool][needless] I wrote can suggest alternatives like this. Applying them
is technically automatable. But I find manually doing tasks such as fixing up
indentation for multi-line functions or updating their use site to be easier.
Your conclusion may depend the size of your code base and your patience.

## Step 3: Audit ##

The main goal for the API Design Guidelines is "clarity at the point of use".
Our automated process will yield some undesirable results. I'll list a few
scenarios here.

__Poorly named function name becomes poorer__. Example:
`alertForScreenSize(_ size CGRize)` becomes `alert(forScreen size: CGSize)`.
Obviously, "for screen" is misleading since a `CGSize` is not a screen. The
problem here is the "alert" is not for a "size" to begin with. We as human
intuitively pick up that it's for a "screen with a certain size". So the correct
renaming here should be `alertForScreen(with size: CGSize)`.

__Awkward literal values results in ungrammatical phrases__. Imagine our
argument is a enum:

```swift
enum Mode {
  case add
  case edit
}
```

… and we renamed a function `updateUIForMode(mode: Mode)` to
`update(for mode: Mode)`. The call site, therefore, becomes
`update(for: .add)`.

What makes it feels wrong? Well, a preposition followed by a verb ("for add")
is not very grammatical. Normally we would say "update UI for *adding*". So here
we need to update the literals in that enum to "adding" and "editing".

__Type information becomes too weak__. This can happen very often with enum
literals. We often name enum values with an adjective to qualify its name:

```swift
enum UIControlState {
    case automatic // which state? the *automatic* state!
    …
}
```
After renaming, we'll have functions that simply becomes too generic and
mysterious at call site:

```swift
update(for: .automatic) // automatic what? 😂
```

This is where we need to be flexible and use our own good judgement. Perhaps
it's simply better to keep the qualifying postfix:

```swift
update(forControlState: .automatic) // better
```

In the guideline's parlance, this is [compensate for weak type
information][weak type information].

[weak type information]: https://swift.org/documentation/api-design-guidelines/#weak-type-information

__Other arugment labels needs updates too__. A function at call site should read
like a sentence as much as possible. We removed words at beginning of the
"sentence", it's important to take a look at labels for the rest of the sentence
and ensure the whole thing fits together.

***

There are *many*, *many* other fallouts from mechanically omitting needless
words that I didn't cover in this post. There are many many other things in the
API Design Guidelines that are worth conforming to. There will be much much bike
shedding among your team members.

Just remember, the API guidelines don't provide an answer to every naming
desicion. Following it means your code base will fit in with the rest of the
community and APIs from Foundation/Swift standard library. But find what feels
right for your team is the most important thing. In the end, you should be glad
that everyone cares so much about your code base's "Swiftness" and it'll be all
worth it!
