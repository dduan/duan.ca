---
title: Making TUI Applications In Swift
tags: [Swift, TUI]
date: 2016-12-26 21:39:05-0800
---

One of my fondest memory in programming is writing the game Snake on one of
these:

![An Electronic/Digital Dictionary]({{site.url}}/assets/2016/12/e-dictionary.jpg)

At the time, I was in a very restrictive school where a real computer wasn't
permited in class. The actual thing (that's an "electronic dictionary" by the
way) was probably older, slower and of a different brand than the one in the
photo. In fact, I didn't know how to make a copy of the code, and the device
wasn't even mine. So the game just stayed there when the device was returned to
the owener and its fate unknown.

But I love it nontheless. Design the game, write it, playing it, showing it off
- everything.

<br />

Fastforward to today. I use and write software applications for a living.
Programs I use directly range from terminal commands to GUI apps and web apps in
a browser.

Some programs, however, don't belong to those categories: vim, tig, irssi, etc.
They run in a terminal like CLI commands. But unlike commands like `ls`, they
take over the entire buffer to offer different modes of interaction. You can
use hotkeys to navigate menus, trigger functions and so on.  Sometimes you can
use mouse, too. Their layout often resemble that of a GUI application running in
it's own "window":

![tig]({{site.url}}/assets/2016/12/tig.png)

Just to make things painfully clear, these apps's interface is composed
exclusively of text. We can vaguely categorize them as "Text-based User
Interface(TUI)" application.

There are many reasons to like TUI applications. Compared to a CLI program,
they can display information in 2-dimentional, more organized layout (aka more
like a GUI app). Unlike a GUI application, they almost only require a terminal
to run. If you ssh onto a Linux server on your iPad, they'd work the same as
running locally on a Mac. For people who live in terminal simulators, it's nice
to run apps that don't open a new Window. Oh, they often tend to look the same
no matter how your OS changes.

Most importantly, they remind me of the programs I built on that electronic
dictionary. I want to make them today, with my favorite programming language
Swift!

<br />

The go-to library for writing TUI apps is [ncurses][]. Classics such as vim,
emacs and mutt were written with it to some degree. But I find it hard to dive
into: it has a large number of strange legacy function names and offers
pre-built UI elements like panels, menus, forms etc. The terminal in my mind is
a spread of characters that can be updated by me. That sounds fun. Ncurses seems
more like UIKit :P.

Luckily, I found the fun library: [termbox][] by [nsf][]. Termbox offers around
a dozen functions that gives us size of the terminal, a way to update text at
a row/column coordinates, and user inputs in forms of key-presses and
mouse-clicks. And that's it!

After making [a Swift wrapper][wrapper], I was able to build a painter app in
[a handleful of lines][terminal-paint]:

![terminal paint]({{site.url}}/assets/2016/12/terminal-paint.png)

So that's an TUI app drawing according to mouse clicks built with only a few
primitives from termbox. Awww yeah! Here's the code with the comment stripped,
to give you a taste:

```swift
import Termbox
func printAt(x: Int32, y: Int32, text: String,
    foreground: Attributes = .default,
    background: Attributes = .default)
{
    let border = Termbox.width

    for (c, xi) in zip(text.unicodeScalars, x ..< border) {
        Termbox.put(x: xi, y: y, character: c,
            foreground: foreground, background: background)
    }
}

func updateHelp(drawing: UnicodeScalar) {
    let lastY = Termbox.height - 1
    let content = [
        "Drawing [\(drawing)]",
        "Press other character to change",
        "Use mouse to draw",
        "Press 'q' to quit"
    ].joined(separator: " | ")

    let filler = String(repeating: " ",
        count: Int(Termbox.width) - content.unicodeScalars.count)

    printAt(x: 0, y: lastY, text: content + filler,
        foreground: .white, background: .blue)
}

func paint() {
    do {
        try Termbox.initialize()
    } catch let error {
        print(error)
        return
    }
    Termbox.inputModes = [.esc, .mouse]

    var drawingCharacter: UnicodeScalar = "." {
        didSet {
            updateHelp(drawing: drawingCharacter)
        }
    }

    updateHelp(drawing: drawingCharacter)

    Termbox.present()

    outer: while true {
        guard let event = Termbox.pollEvent() else {
            continue
        }

        switch event {
        case let .character(_, value):
            if value == "q" {
                break outer
            }

            drawingCharacter = value

        case let .key(_, value):
            if value == .space {
                drawingCharacter = " "
            }

        case let .mouse(x, y):
            Termbox.put(x: x, y: y,
                character: drawingCharacter,
                foreground: .red)
        default:
            continue
        }

        Termbox.present()
    }

    Termbox.shutdown()
}

paint()
```

[Library][wrapper] and this [app][terminal-paint] are both on Github. Go and
have fun :)

[ncurses]: https://www.gnu.org/software/ncurses/
[termbox]: https://github.com/nsf/termbox
[nsf]: https://github.com/nsf
[wrapper]: https://github.com/dduan/Termbox
[terminal-paint]: https://github.com/dduan/TerminalPaint
