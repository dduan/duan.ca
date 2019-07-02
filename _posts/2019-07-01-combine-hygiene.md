---
title: On the Subject of Interface Hygiene
tags: [Combine, Swift]
date: 2019-07-01 20:22:48-0700
---

In a purly reactive world, your entire program should be a single stream. Now,
close your eyes, and envision: your project as one, beautiful, stream.

Now open your eyes. Yeah, it's not. Your project is a Mac or iOS app. It's full
of your memories, sweat, blood. Now you are ready to sweat and bleed some more
by putting some Combine and SwiftUI into it. You watched the WWDC19 sessions and
learned that "Subjcets are super powerful". You looked into your code and
realized you can't really do anything with Combine without `Subject`s at the
current state of the project.

Well…

Here are a few habits that help keeping your project that prevasively uses
`Combine.Subject` *sane*. They should seem obvious to anyone who understands
murphy's law and the value of minialism in interfaces. If you already are using
some reactive stream implementation, substitute the types with their
counterparts in your framework and these rules should seem down right **basic**.

## Disguise `Subject`s as `Publisher`s

`Subject`s help bridge from the imperitive to the reactive world. Somewhat
paradoxically, sharing them is not very "RX-y". This is akin to prefering `let`s
over `var`: sharing the stream, but not the previlage of mutating it.

Most of the time, what you want to share is the values pumped into the stream.
Because Subjects conform to `Publisher`, it's easy to hide from the users the
fact that your stream is backed by them. With Combine this conversion
happens via type-erasure:

```swift
// Bad: now anyone who get a hold of it can mess with your stream!
public enum GreatInts {
    public var updates = CurrentValueSubject<Int, Never>(0)
}
```

```swift
// Better: all your users care is the stream (publisher), so give them that!
public enum GreatInts {
    // Internally, it's backed by a subject.
    var subject = CurrentValueSubject<Int, Never>(0)
    // Externally, it's just a Publisher. 
    public var subject: AnyPublisher<Int, Never> {
        subject.eraseToAnyPublisher()
    }
}
```

## CurrentValueSubject natually caches the latest value

RX theorists will hate this: sometimes it's just practical to expose
a synchronous interface to the latest vaule in the stream!

Two things.

1. It might be tempting to expose the subject and let your user use its
   `.value`. Well, you shouldn't (as explained in the previous section).
   A separate interface dedicated to the latest value prevents people from
   polluting your stream.

```swift
// (Still) bad
public final class GreatInts {
    public var updates = CurrentValueSubject<Int, Never>(0)
}
```

2. Remember `CurrentValueSubject` has that `.value` property! It may seem
   surprising, but I've seen folks transitioning to RX clinging to the old ways:

```swift
public final class GreatInts {
    // well, at least it's not a public subject...
    var subject = CurrentValueSubject<Int, Never>(0) // <- initial value 0
    public var updates: AnyPublisher<Int, Never> {
        subject.eraseToAnyPublisher()
    }

    // Wait, there's that 0 again
    public var latest: Int = 0 {
        didSet {
            subject.send(latest) // ?
        }
    }
}
```

First, you'll notice that `0`, the initial value, is duplicated as both the
subject's initial value, as well as the value of a stored property. And these
duplicated sources of truth persist throughout the parent's life time. Weird,
right?

Here's a slightly better version:

```swift
public final class GreatInts {
    var subject = PassthroughSubject<Int, Never>()
    public var updates: AnyPublisher<Int, Never> {
        subject.eraseToAnyPublisher()
    }

    public var latest: Int = 0 {
        didSet {
            subject.send(latest) // ?
        }
    }
}
```

Now there's no two copy of the latest value in memory anymore. But in my opinion
it does not embrace the full power of Combine. Here's the most natual way to do
this:

```swift
public final class GreatInts {
    /// This is a CurrentValueSubject again.
    var subject = CurrentValueSubject<Int, Never>(0)
    public var updates: AnyPublisher<Int, Never> {
        subject.eraseToAnyPublisher()
    }

    public var latest: Int {
        subject.value
    }
}
```

Essentially, you create separate public interface, each vends a little piece of
`CurrentValueSubject`'s power.

## No really, don't use `Subject`s

Even a well-scoped `Subject` (properly being private or internal, depending on
your tool of choice for access control) still has a mutable state that you
probably don't want: its stream can go from "alive" to "complete". And, again,
anyone with access can make this state transition happen, leaving you in the
undefensible position of … hoping everyone on your team to not misuse your
stuff?

Lucky for you (and me), a "incompletable" subject is a established concept -- a
"Relay". I've put together a repo for you to look and/or use:

[https://github.com/dduan/relay](https://github.com/dduan/relay)

Yeah, ban all Subjects in your project with a linter. Seriously.

## Fin

That's all for now. I'm not an expert with RX myself. Hopefully these
perspective can help you avoid some nasties.
