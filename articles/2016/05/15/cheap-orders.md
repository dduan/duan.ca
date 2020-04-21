# Cheap Orders
2016-05-15T17:42:27-07:00
tag: Swift

How to create order? If the second law of thermal dynamics tells us anything,
we'd better get to work, right?
{: .lead}

Before going full existential, let's limit "order" to Swift's set and
dictionaries -- there is none. Of course, you can take values/keys out and
sort them. But what if all you care about is *some* order?

<hr />

Recently, I wrote

```swift
enum Token {
  static let all = [
    "=": .Equal,
    "-": .Minus,
    // more token mappings …
  ]
  case Equal
  case Minus
  // more tokens …
}
```

… in the hope that I can take each value from `Token.all.keys` and see if
a prefix of a string is a matching token. It started to fail as the tokens
expands to multiple characters:

```swift
enum Token {
  static let all = [
    "=": .Equal,
    "-": .Minus,
    // more token mappings …
    "==": .Equality,
    "->": .Arrow,
    // more token mappings …
    "===": .Identity,
    // more token mappings …
  ]
  case Equal
  case Minus
  case Equality
  case Arrow
  case Identity
  // … more tokens …
}
```

`->` could get a match with `-` and `===` would match to either `==` or `=`,
etc.

Since the tokens in this exercise have at most 3 characters, I decided to
group them by length and match from the longer group first. The groups became:

```swift
enum Token {
  static private let group1 = [
    "=": .Equal,
    "-": .Minus,
    // more token mappings …
  ]

  static private let group2 = [
    "==": .Equality,
    "->": .Arrow,
    // more token mappings …
  ]

  static private let group3 = [
    "===": .Identity,
    // more token mappings …
  ]
}
```

Now I can choose which group to take values first. There's a way to do it
without adding some control flow logic:

```swift
[group3.keys, group2.keys, group3.keys].flatten()
```

Even better, I'll make it a lazy property…

```swift
enum Token {
  static var all = {
    [group3.keys, group2.keys, group3.keys].flatten()
  }()
}
```

…except an important piece of information is missing from the property: what's
`all`'s type? Turns out, it's become

```swift
FlattenCollection<Array<LazyMapCollection<Dictionary<String, Token>, String>>>
```

Ahh, it seems that in the pursue of cheap, lazy creation of these structures,
we are forced to deal with a bunch of type wrappers, each having a good reason
to be here!

But I really just need something like `Array<String>` for the consumer. If
only there's a way to make all this stuff go away from my type signature, as
if they are [erased](http://robnapier.net/erasure) :).

Okay, I'm talking about `AnySequence` now. Rob Napier has an excellent post on
this topic [here](http://robnapier.net/erasure) if you need to catch up. Our
code eventually end up like this:

```swift
enum Token {
  static var all: AnySequence<String> = {
    AnySequence(
      [group3.keys, group2.keys, group3.keys]
        .flatten())
  }()
}
```

Instead of `Array<String>`, we have an `AnySequence<String>`. Our tokens now
gets checked with the correct order. We didn't need to sort the entire set of
tokens, nor did we do any heavy data massage upfront, making a bunch of copies
along the way.

<hr />

Looking back, this post really failed to capture the eureka moment as I came
up with the erasure method. I discovered a series of small challenges and got
help from Swift's designers in each step. Everything fell together in the end.
