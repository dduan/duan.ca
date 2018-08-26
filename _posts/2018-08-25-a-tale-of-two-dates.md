---
title: A Tale of Two Dates
tags: [Swift, Foundation]
date: 2018-08-25 17:11:43-0700
---

Recently, I discovered a curious thing about `Date`s in two large projects
I work on. Simply put, both projects receives, from various HTTP endpoints, the
same object component: a timestamp, and a duration. Combining these two pieces,
both projects derives two `Foundation.Date`s two represent a time range. So far,
so good.

However, project `A` uses `Fonudation.DateInterval` to represent this concept,
while project `B` uses `Range<Date>`. But why? Why represent the same component
differently? What a gigantic waste of brain power for everyone on both projects!

So I set out to unify this thing. Wherever a `Range` literal is used, I swap in
`DateInterval.init(start:end:)`; `Range.lowerBound` becomes
`DateInterval.start`; `Range.upperBound` becomes `DateInterval.end`, etc. It
didn't take long to complete the conversion to `DateInterval` in project `B`,
now it builds and runs!

Oh, wait, why are some tests failing in probject `B`? Shouldn't this just be an
mechanical change?

I spent time investigating. The failing tests are for some very specific
business logic that I'm not familiar with. So things took a while to become
clear. What felt like a long time later, I realized my mistake.

(I'm sorry if this has been obvious to you. You are a better Swift programmer!)

Somewhere in project `B` is the following:

```swift
struct Item {
    let range: Range<Date>
}

struct Container {
    let items: [Item]
}

func containerFactory(range: Range<Date>, items: [Item]) -> Container {
    /// pretend there's more code here

    return Conatiner(items: items.filter { $0.range.contains(range.lowerBound) })
}
```

And of course, after my "refactor", it became

```swift
struct Item {
    let range: DateInterval
}

struct Container {
    let items: [Item]
}

func containerFactory(range: DateInterval, items: [Item]) -> Container {
    /// pretend there's more code here

    return Conatiner(items: items.filter { $0.range.contains(range.start) })
}
```

Tests for `containerFactory` failed. And here's why: **`DateInterval.contains`
inclusive for its upper bound (`.end`), whereas `Range.contains` isn't!** You
can see it plainly by runnig the following

```swift
import Foundation

let sooner = Date(timeIntervalSince1970: 0)
let later = sooner.addingTimeInterval(1000)

DateInterval(start: sooner, end: later).contains(later) // true
(sooner..<later).contains(later)                        // false
```

So here's what stumped me:

1. The 2 projects chose to interpret the same component differently, which
   I didn't not expect.

2. I didn't know how `Foundation.DateInterval` works.

Well, today I learned.
