# Faster C Strings in Swift
2020-09-09T01:21:26-07:00
tag: Swift, C, Pathos

One of the goals in the re-write of my VFS library [Pathos][] is to make it go
fast. What does that mean when most of the time users are hitting the hard
drive when running your code? Well, let's not dwell on that for now.

A library like this passes *file paths* back and forth with C APIs from the OS
a lot. So one way to go fast is to keep the original representation of the paths
as they are provided to us. On macOS and Linux (and other OSes that Swift
doesn't officially support yet), paths are bought and sold in the so called
"C strings": `NUL`-terminated bytes (8-bit integers) with POSIX APIs and 16-bit
values on Windows with `UNICODE`.

Let's walk over a couple of examples. Here's how to get the current working
directory:

```c
// POSIX
char *getcwd(char *buf, size_t size);

// Windows
// LPTSTR (with right environment) is `wchar_t *`
DWORD GetCurrentDirectory(
    DWORD nBufferLength,
    LPTSTR lpBuffer
);
```

The expected steps for using APIs like this are the following:

1. you allocate a buffer large enough to store any result you expect.
2. you give the buffer to the C function.
3. the C function fills the buffer with some characters, and a `NUL` (or `0`)
   right after the last character in the result.
4. the function use a separate variable to tell you the size of the actual
   result, not counting the `NUL`.

It's very generous of these functions to give us the size of the result. Because
the point of using `NUL` to terminate "strings" is to avoid having to use
a separate size. Let's use *setting the current working directory* as the next
example:

```c
// POSIX
int chdir(const char *path);

// Windows
BOOL SetCurrentDirectory(LPCTSTR lpPathName);
```

Yup, these APIs don't need you to tell them the content size of your buffer. But
if your content doesn't end with a `NUL`, they'll happily read beyond your
intended stopping point until it finds a random `0` in memory.

Anyways, this is pretty classic C stuff. Now let's talk about Swift!

The default option to store a file path for most should be a `Swift.String`,
which is a encoding-agnostic, Unicode glyph based list of characters. But as
I mentioned earlier, I want to avoid the cost of decoding and encoding in the
case where the path is only passing through the Swift code from C to C, without
being analyzed or modified. (If you need a refresher, [I recently made a video
on Unicode and encoding][UnicodeVSUTF8].) So I chose to use an
[ContiguousArray][] to store these C values:

```swift
// Simplified for POSIX
struct Path {
    let storage: ContiguousArray<CChar>
    // ...
}
```

`ContiguousArray` (and `Array`) provides an excellent way to interact with C
APIs we encountered earlier:

```swift
init(
    unsafeUninitializedCapacity: Int,
    initializingWith initializer: (
        inout UnsafeMutableBufferPointer<Element>,
        inout Int
    ) throws -> Void
) rethrows
```

Don't let the complex-looking signature intimidate you. Essentially, it gives
you write access to the array's memory right after its allocation, but before
Swift does standard initialization to it. It works really well with the C APIs
we looked at earlier:

```swift
// Store the current directory in a ContiguousArray
// Using the Windows API
let storage = ContiguousArray(
    unsafeUninitializedCapacity: Int(MAX_PATH)
) { buffer, count
    let length = GetCurrentWorkingDirectoryW(
        DWORD(MAX_PATH),
        buffer.baseAddress // C API writes in the allocated memory
    )

    count = length // you are responsible for setting size of the array
}
```

Read the steps 1-4 again from earlier, it's easy to see how this initializer is
designed to fit that pattern. The resulting array will have all the characters
as its content, and carries the correct size.

When it's time to pass the array back to C, we can provide a pointer easily:

```swift
storage.withUnsafeBufferPointer {
    SetCurrentDirectory($0.baseAddress!)
}
```

This is not great, because we don't have a `NUL` at the end of our array.  The
C function that read our array will sometimes read over the contents memory
until it finds a 0! Yikes.

So here's an easy fix:

```swift
(storage + [0]).withUnsafeBufferPointer {
    SetCurrentDirectory($0.baseAddress!)
}
```

Instead of using the memory of `storage`, we construct a new array with an 0 as
its last value. This lets C APIs pick the right place to stop reading.
(Incidentally, Swift includes a built-in version of this [for converting String
to UTF-8 (8-bit) C strings](https://developer.apple.com/documentation/swift/string/2430818-utf8cstring),
which includes the `NUL` and it's possible to further encode with different
encodings.)

Although we've fixed the correctness bug, doing this defeats the purpose of
storing the C string directly somewhat: constructing a new array each time we
want to call a C API is kind of expensive. It involves allocating new memories
and copying over the content, etc.

Alright. How about we carry around the `NUL` in our array? Let's update the
construction code:

```swift
let storage = ContiguousArray(
    unsafeUninitializedCapacity: Int(MAX_PATH) + 1
) { buffer, count
    let length = GetCurrentWorkingDirectoryW(
        DWORD(MAX_PATH),
        buffer.baseAddress
    )

    buffer[length] = 0
    count = length + 1
}
```

We add 1 every time we have a say in size. Then we manually set a 0 at the end
of the stuff from C. Having done this, we've solved both the correctness problem
and performance concern from earlier!

The last bit of of this journey is ergonomics. Carrying an extra `NUL` is fine
if you never look at the array's content. But when you do, it's important to
remember that the content we care about is *almost* all of the array, except for
the `NUL` at the end. In other words, simply don't make off-by-1 mistakes and
everything will be fine.

Alright, that's easier said than done. To alleviate this off-by-1 painfulness,
I ended up exposing a "view" into the array storage that excludes the last
element. Here's the actual definition in [Pathos][]:

```swift
struct CString<Unit: BinaryInteger>: Equatable, Hashable {
    private var storage: ContiguousArray<Unit>
    var content: ContiguousArray<Unit>.SubSequence {
        storage[0 ..< storage.count - 1]
    }

    public func c<T>(
        action: (UnsafePointer<Unit>) throws -> T) throws -> T
    {
        try content.withUnsafeBufferPointer {
            try action($0.baseAddress!)
        }
    }

    init(cString: UnsafePointer<Unit>) {
       var length = 0
       while cString.advanced(by: length).pointee != 0 {
           length += 1
       }

       storage = ContiguousArray(
           unsafeUninitializedCapacity: length + 1
       ) { buffer, count in
           for offset in 0 ..< length {
               buffer[offset] = cString.advanced(by: offset).pointee
           }

           buffer[length] = 0
           count = length + 1
       }
    }

    // ... more stuff
}
```

`storage` in this solution is an private implementation detail. `content` is
the primary access to the content of the string. And finally, this type
interops with C APIs correctly and efficiently because of the extra `NUL` we put
at the end of `storage`.


[Pathos]: https://github.com/dduan/Pathos
[UnicodeVSUTF8]: https://youtu.be/Vy2r21kli0Q
[ContiguousArray]: https://developer.apple.com/documentation/swift/contiguousarray

