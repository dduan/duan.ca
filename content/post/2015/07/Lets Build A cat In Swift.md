+++
date = "2015-07-04T20:56:19.429925-07:00"
title = "Let's Build A 'cat' In Swift 2"
slug = "lets-build-a-cat-in-swift-2"
+++

As a homework in one of the early college classes, I was asked to write unix
commands such as `cat` in C. Let's do that in Swift today! To make things
interesting, let's pretend we are on Linux. That means no Xcode nor Foundation
can be used.

It's hard to find a simpler unix program than `cat`: It takes a list of file
names from the shell and write the content of each file to `stdout`. When no
argument is given, it uses `stdin` as the source of its output.

Writing it in C is trivial. Swift has exellent support for leveraging C. But
to call even the standard C functions, we need to import them first.

The `swiftc` command can compile a pure Swift source file like this:

    swiftc cat.swift -o cat

We can add Objective-C bridging headers with the argument
`-import-objc-header`.  But to import the standard C functions, we also need
to specify path to an SDK:

    swiftc -sdk $(xcrun --show-sdk-path --sdk macosx)\
           -import-objc-header bridge.h\
           cat.swift\
           -o cat

Instead of typing/copying that command, save this `Makefile` to the same
directory as `cat.swift`:

    SDKPATH = $(shell xcrun --show-sdk-path --sdk macosx)
    CBRIDGEHEADER = bridge.h
    TARGETS := cat

    .PHONY : all $(TARGETS)

    all: $(TARGETS)

    $(TARGETS):
        swiftc -sdk $(SDKPATH) $@.swift -import-objc-header $(CBRIDGEHEADER) -o $@

Now `make cat` should take care of the compilation.

Since file I/O is the only concern, we'll need C APIs from `stdio.h`, so
`bridge.h` is a one liner:

    #import <stdio.h>

The standard C function for opening a file is `fopen`:

    FILE * fopen ( const char *filename, const char *mode );

Hmmmm, how do we deal with all those pesky '*'s?

To reference a certain C `Type` in Swift, we can use `UnsafePointer<Type>` or
`UnsafeMutablePointer<Type>`. To make our lives easier, Swift `String`s
automatically bridge to `const char *`. In other words, we can treat the
signature of `fopen` as if it's the following:

    func fopen( filename: String, mode: String ) -> UnsafeMutablePointer<FILE>

A character in C is represented by a byte in memory. Therefore Swift sees
a `char` as of type `Int8` (8-bit integer).  So a `char *` would be referenced
as `UnsafeMutablePointer<Int8>` in Swift. So `getline`, a function from POSIX

    ssize_t getline( char **lineptr, size_t *n, FILE *stream );

would look like this in Swift:

    func getline(
        inout lineptr: UnsafeMutablePointer<Int8>,
        inout n: UInt,
        stream: UnsafeMutablePointer<FILE>
    ) -> Int

It returns the number if characters it finds.

We now can open a file, read and print its content line by line, and close it
with:

    func fclose(stream: UnsafeMutablePointer<FILE>)

Repeat this on each file specified in `Process.arguments`, or simply read from
`stdin`, and we have a `cat`! Here's a screenshot of it displaying its own
code:

![Swift cat](/images/2015/07/swift-cat.png)

The code is also available in this [gist][].

[gist]: https://gist.github.com/dduan/f6d359019db8b0b55962
