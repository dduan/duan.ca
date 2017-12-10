---
title: List Comprehension In Swift
tags: [Swift, Python, Haskell]
date: 2017-12-09 12:26:30-0800
---

Let's explore ways to add list comprehension to Swift.

## Motivation

[List comprehension][0] should be no stranger to a Python or (and?) Haskell user. It's a really compact syntax
that deals with [Cartesian product][1] of lists. In the case of Python, it's probably responsible for the lack
of evolution of lambda expressions, since it's much nicer to write one-liners with it in place of `map`s and
`filter`s.

Here's an example of an list comprehension in Haskell from Wikipedia:

```haskell
a = [(x,y) | x <- [1..5], y <- [3..5]]
-- [(1,3),(1,4),(1,5),(2,3),(2,4) ...
```

In this example, a list of pair of integers is constructed from 2 lists of integers.

Here is what that example would be in Python:

```python
a = [(x, y) for x in range(1,6) for y in range(3, 6)]
# [(1, 3), (1, 4), (1, 5), (2, 3), (2, 4) ...
```

Here's what it would be in mathematics (except we are dealing with sets, not lists, but I'll only refer to
lists from here on.):

```
Let (a, b) be an ordered list of elements

{(x, y)|x ∈ {1,2,3,4,5}, y ∈ {3,4,5}}
```

One can filter out unwanted elements with predicates, and apply arbitrary functions to elements of the
result. Let's say we only want even numbers from the first list, and we want the sum of x and y, continuing on
our examples:

```haskell
a = [x+y | x <- [1..5], y <- [3..5], x `mod` 2 == 0]
```

```python
a = [x + y for x in range(1,6) for y in range(3, 6) if x % 2 == 0]
```

```
{x+y|x ∈ {1,2,3,4,5}, y ∈ {3,4,5}, x is even}
```

In theory, this syntax can be applied to an arbitrary number of lists. Putting aside how often this need comes
up in day-to-day programming in your domain, it should be obvious that it's alternative, be it nested loops or
`map`s and `filter`s, is pretty clumsy in comparison.

## Adding List Comprehension in Swift

A comprehension can be considered in 3 parts:

1. some lists, each may contain a different type of elements.
2. a predicate (or a series of them joined logically) to filter out elements.
3. a function to process the combination of elements into results.

In Swift, if our input is only one list, there's a pretty sweet way to achieve that:

```swift
list.filter(predicate).map(processor)
```

To make comprehension work with more lists, we have some syntax options.

### Option One

The "brute force" option would be a function that parameterize all 3 parts of the comprehension. Such as

```swift
// going with order of appearance in Python/Haskell syntax
func comprehension<Element, List, Result>(
    predicate: (Element) -> Bool,
    list: List,
    processor: (Element) -> Result
) where
    List: Sequence, List.Element == Element

{
    // implementation
}
```

To supporting more than one list, just add more parameters to both types and the function itself.

(Can't wait until we can have [variadic generic parameters][2]!)

### Option Two

Deploy more syntax tricks. Somehow make it visually similar to the math/Haskell/Python notation. If we can
accept some temporary data structure and introduce/implement some operators, there'd be many possibilities.

```swift
/// Just an example of the infinite possibilities.
processor | list0 &&& list1 | predicate |
```

I'll leave the implementation of this example as an exercise to the reader.

### Option That I Like

I spent quite some time exploring the realm of possibilities in "option two". However, introducing data
structures and custom operators just to do what "option one" offers seems really unappealing. It's not
entirely clear that doing so would be "Swift-y" anyways! Eventually, I did find an arrangement that fits in
Swift, and requires no fancy syntax trickery.

The result of list comprehension is a list. The goal of this operation is to _construct_ a list. Yep, thinking
along this line, it became obvious that using a "list"'s initializer is just natural:

```swift
let a = Array(1..<5, 3..<5, where: { n, _ in n % 2 == 0 }) { ($0, $1) }
// [(2,3),(2,4),(2,5) ...
```

The processing function is at the end to take advantage of the trailing closure syntax. It's nicer when
there's not predicate:

```swift
let a = Array(1..<5, 3..<5) { ($0, $1) }
// [(1,3),(1,4),(1,5),(2,3),(2,4) ...
```

This syntax seems both succinct and Swift-y.

I put an implementation on [github][3], in case you find it useful.

## Parting Thoughts

There's no doubt that the conclusion in this post is imperfect. Though it feels more Swift-y, it deviates from
the mathematical syntax by a lot. We can only implement it for finite number of lists. When many lists are
involved, using a embedded closure as the predicate would make the compiler complain that the expression is
too complex. We suffer from the normal woes with Swift closures where anonymous arguments (`$0`, `$1`, etc)
won't work unless the last one is mentioned in the closure's body. Overloading `Array` initializer may
negatively affect compilation speed in large projects.

Not all of these issues are temporary.

Does list comprehension warrant a language change in Swift? Can you think of better ways to implement it
with the current compiler?

[0]: https://en.wikipedia.org/wiki/List_comprehension
[1]: https://en.wikipedia.org/wiki/Cartesian_product
[2]: https://github.com/apple/swift/blob/master/docs/GenericsManifesto.md#variadic-generics
[3]: https://github.com/dduan/Comprehension
