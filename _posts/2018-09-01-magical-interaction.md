---
title: Magical Interactions
tags: [Swift, Haskell]
date: 2018-09-01 11:08:57-0700
---

I want to talk about this little function:

```swift
func interact(_ process: (String) -> String) {
    var input = ""
    while let line = readLine() {
        input += line
    }

    print(process(input))
}
```

Brief explanation: it reads all the input from stdin as a `String`, feeds it
into a closure `process`, which it takes in as its only argument, and prints
`process`'s output.

Here's how one might use it:

```swift
// counts characters from stdin and prints result to stdout
interact { String($0.count) }
```

Got that? Well, now I'm going to rewrite it in a slightly less Swift-y way:

```swift
let count: (String) -> String {
    return String($0.count)
}

interact(count)
```

The argument for `interact` got defined with a name and an explicit type
signature.

So, what's so special about this `interact` function? Two words: *side effects*.
More precisely, it took away the concern of side-effects from the user. `count`
belongs in the realm of *pure* functions. It has no worries about file handles
or operating systems. It's `(String) -> String`. I wanted to emphasize this with
the rewrite. Look at that empty line. Now you see a boundary between 2 worlds.

This may all seem contrived. But when I learned about [this function in
Haskell][0], I was blown away.

It's like a great magic trick: you are presented a scenario, say, writing
a little script. Maybe you need to process some CLI output and print out a CSV
or JSON string (literally 90% of the script I write).  A Haskell programmer
would jump into the bottom level of the problem and start writing these little
pure functions: one to split the string, one to convert some numbers, one to
manipulate a list, one to match some patterns...gradually the broken-down
absractions get built back up via function composition. You can see the light at
the end of the tunnel, yes, yes! If you feed this list into that function that
returns a string you'll have the right value to print out!  Okay, now the
problem is solved in the pure functional world! The only thing left to do is...

Now, the setup of the magic is complete. Now, you are onboard with the solution,
you thought the problem through with the magician...you are distracted. The
ending came so...quickly, but unexpected. What? You just feed your solution to
this `interact` function and...that's it? I was expecting some `readLine`s or
`print`s (okay, at least 1 `print` statement)!

That's the thing: `interact` deals with _two_ side effects, the input and the
output. But its user deals with _zero_. It's as if the two effects "cancel" each
other out! It's a _neat_ trick, really. Small, low-key, easy to miss. But I'm
glad I noticed it and come to appreciate its power and simplicity.

[0]: http://hackage.haskell.org/package/base-4.11.1.0/docs/Prelude.html#v:interact
