---
title: Swift Algebraic Data Types
tags: [Swift]
date: 2015-07-12 15:51:27.020057-07:00
---

## The Basics

[Algebraic Data Type][] is a fancy name for "a type creaded by combining other
types" in programming languages. One aspect of the "algebraic-ness" is how
many potential new values there are for the new type, given a set of type as
its building block.

To better illustrate this, consider the following example in Swift.

```swift
enum StarkChild { case Rickon, Bran, Arya, Sansa, Robb, Jon }

enum Direwolf { case Shaggydog, Summer, Nymeria, Lady, Greywind, Ghost }

enum Actor {
    case Wolf(Direwolf)
    case Person(StarkChild)
}

struct Parters {
    var wolf: Direwolf
    var person: StarkChild
}
```

`StarkChild` and `Direwolf` each has 6 potential values. Combining them, we
get two new types.

Question: how many potentiol values are there for `Actor`? How many for
`Parters`?

<hr>

An `Actor` can be either a `StarkChild` or a `DireWolf`, therefore it has `6
+ 6 = 12` potential values – the *sum* of `Child`'s and `DireWolf`'s values.

A `Partners` requires us to select one value from `Child` and one from
`DireWolf`, resulting in `6 * 6 = 36` potential values – the *product* of
`Child`'s and `DireWolf`'s values.

`Actor`, an `enum`, is a *sum* type. `Parters`, a `struct`, is a *product*
type. Here, `Parters` could easily be defined as a `class` or a `tuple` and
remain a *product* type. Because we can create product or sum types in these
direct ways, we can say Swift has first class support for Algebraic Data
Types.


## The Crossovers

However, the story doesn't stop here. In Swift, an `enum`'s option can have
multiple values. If it happens to be the only option, then this
`enum`effectively becomes a *product* type:

```swift
// there are 6 * 6 = 36 potential values for Parters
enum Parters {
    case Value(wolf: DireWolf, person: StarkChild)
}
```

Incidentally, this makes `enum` similar to `data` in Haskell, where *product*
and *sum* types can be created with a unified construct – `data`.

In C and C++, `union`s are the closest thing to *sum* types. However, `union`
is hardly used to combine arbitrary types due to its lack of associated
values. What do people do in need of *sum* types? They make do with product
types. Here's one way to achive that in Swift:

```swift
// Actor.value can have only 6 + 6 = 12 potential values thanks to
// manual enforcement
class Actor {
    var child: StarkChild?
    var wolf: Direwolf?

    var value: Any {
        get {
            return child == nil ? wolf! : child!
        }
        set(newValue) {
            if newValue is StarkChild {
                child = (newValue as! StarkChild)
                wolf = nil
            }
            if newValue is Direwolf {
                wolf = (newValue as! Direwolf)
                child = nil
            }
        }
    }

    init(wolf: Direwolf) {
        self.wolf = wolf
    }
    init(child: StarkChild) {
        self.child = child
    }
    init() {
        fatalError("must initialize with a child or a wolf")
    }
}
```

It's… ugly.

## Recursion Types

Besides *sum* and *product*, another common class of algebraic type is
recursion types. The interesting bit here is that Swift struggles to support
it. In WWDC 2015, it was announced that `enum`s can be defined recursively in
Swift 2:

```swift
enum Tree {
    case Empty
    indirect case Node(Tree, Tree)
}
```

As of this writing, Xcode 7 beta 3 has not delivered this feature yet.
Also, it's a good bet that `indirect` is not going to be available in tuple
aliases, such as:

```swift
typealias Node = (indirect Node, indirect Node)
```

I hope this is on the Swift team's (understandably) gigantic todo list 😉.

[Algebraic Data Type]: https://en.wikipedia.org/wiki/Algebraic_data_type
