# Introducing Dye
2020-06-01T15:08:01-07:00
tags: Swift, CLI, Windows

*Recently, I got a PC. And I started writing some code on Windows for the
giggles. Naturally, I gravitate towards stuff I use on macOS and Linux when it
comes to tooling. To my delight, NeoVim, ripgrep and fzf all work out of the box
in terminal simulators, which brings us to todays main topic...*

I made a terminal coloring library for Swift that works on Windows, 
[Dye 0.0.1 is available now][Dye 0.0.1]!

## So, why?

When I started working on [tre](https://github.com/dduan/tre), I search in the
Rust ecosystem for a CLI interface library that supports as many platforms as
possible. Eventually I found [termcolor][] among an ocean of options. As
a result, tre, like a lot of other CLI tools (like rg) written in Rust, has
a consistent UI on Windows and Unix. This experience has brought lots of joy, as
a user of both the library, and the app. I want to pay it forward to my fellow
Swift CLI makers, and their users.

Zooming out slightly, success of Swift on Windows makes Swift as a skill more
valuable. And [I want more CLI tools written in Swift][CLI]. So it's
a double-win, really.

Lastly, it's a small library, all things considered. Being able to get it to
a shippable state on a weekend is a key reason I decided to work on it.

## Technical tidbits

I love Max Howell's [Chalk][] library. It's a 100-line Swift file that
implements [ANSI escape code][ANSI] with Swift's custom string interpolation
API. It demonstrates well how simple it is to customize your terminal output.

Enter Windows, where ANSI sequences are ignored by built-in terminal simulators
from the past. The console is customized via a entirely separate, stateful,
imperative APIs (Newer simulators such as the freshly released [Terminal]()
actually supports ANSI codes pretty well). This is our lowest common API
denominator, which ultimately dictated the design of Dye.

Dye's API is centered around Swift's built-in protocol `TextOutputStream`. You
create a stream object and mutate the style need for upcoming output:

```swift
let output = OutputStream.standardOutput()
output.color.foreground = .blue
print("blue text", to &stream) // blue text is blue
```

If the stream is redirected to something other than the terminal, styling gets
automatically disabled. There are various options to customize this behavior.

Take a look at this [sample app][] to get a more concrete picture of
how things work.

***

I'll end with a screenshot of the sample app running in Command Prompt:

![Dye sample app running in Windows Command Prompt](/assets/2020/06/01/windows-example-screenshot.jpg)

Let's build more.

[termcolor]: https://github.com/BurntSushi/termcolor
[Dye 0.0.1]: https://github.com/dduan/Dye/releases/tag/0.0.1
[CLI]: https://duan.ca/2019/01/20/kick-ass-cli-tools-in-swift/
[Chalk]: https://github.com/mxcl/Chalk
[ANSI]: https://en.wikipedia.org/wiki/ANSI_escape_code
[Terminal]: https://github.com/microsoft/terminal
[sample app]: https://github.com/dduan/Dye/blob/master/Examples/main.swift
