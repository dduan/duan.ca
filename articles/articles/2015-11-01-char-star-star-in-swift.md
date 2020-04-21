+++
title = "char ** in Swift"
date = 2015-11-01T11:18:40-08:00
[taxonomies]
tags = ["Swift", "C"]
+++

A "string" in C is just a continuous chunk of `char` values in memory with
`\0` at the end. To reference it, a variable of type `char *` is used to store
the address of the first `char` (commonly known as a pointer 😉).  It's common
to have string manipulating functions take this form:

```swift
void foo(char **errorMessage) {
    // ...
}
```

To mutate the arugment `errorMessage` of type `char *`, `foo` takes a pointer
to it, `(char *)*`.

<hr />

How do we call `foo` in Swift?

Here's the tl;dr. We can wrap it in a Swift function that have the same
interface:

```swift
func fooSwift(inout errorMessage: String?) {
    var message: UnsafeMutablePointer<CChar> = nil

    foo(&message)
    errorMessage = String.fromCString(message)
}
```

`errorMessage` will contain whatever our C function `foo` assigns to it.

<hr />

So, what's really going on here?

Inspecting `foo`'s signature in Swift, we see

```swift
func foo(errorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>>)
```

… okey, `errorMessage`'s type is little intimidating to someone who doesn't
use C functions in Swift everyday (like me)!

Let's break it down. Swift does a ton of work for us to interoperate with C:

1.  `CChar` is Swift's name for *`char` in C* (shocking, amiright?)

2.  `UnsafeMutablePointer<Type>` roughly translates to `Type *`, so
    syntactically, we can see why
    `UnsafeMutablePointer<UnsafeMutablePointer<CChar>>` is used to bridge the
    C type `(char *)*`.

3.  A function that takes `UnsafeMutablePointer<Type>` argument accepts
    `inout Type` values. Therefore, we can look at `foo` as

        foo(inout errorMessage: UnsafeMutablePointer<CChar>)

4.  Swift acknowledge C's string representation and provides convenient
    methods for converting `char *` / `UnsafeMutablePointer<CChar>` to its own
    `String` type (`String.fromCString()`).

Hopefully you can see how `fooSwift` works now.

<hr />

Taking a step back, to deal with `char **` in Swift, we overcame 2 obstacles:

1.  The difference in string representation between C and Swift.

2.  Compared to C pointers, Swift's `inout` expresses mutability of function
    arguments in a more restricted way. We can't nest `inout`s to represent
    the infinite layers of indirections that pointers achieve.
