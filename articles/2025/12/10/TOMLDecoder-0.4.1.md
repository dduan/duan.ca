# TOMLDecoder 0.4 is 800% Faster
2025-12-10T17:44:34-08:00
tag: Swift, TOML, TOMLDecoder, OSS, Performance

I just released version 0.4.1 of TOMLDecoder,
a TOML 1.0 parser,
and [decoder](https://developer.apple.com/documentation/swift/codable) implemented in pure Swift. 
When decoding a TOMLDocument such as [this twitter payload](https://github.com/dduan/TOMLDecoder/blob/cea8f0bee33f37e0fcc33b566a742485c71196e7/Sources/Resources/fixtures/twitter.toml),
TOMLDecoder 0.4.1 is roughly 800% faster by wall clock time than 0.3.x.
In this post, I’ll discuss how this was achieved.

_tl;dr: among other things,
the gains comes from making the parsing algorithm lazier,
and eliminating overheads from bound checking when accessing substrings._

## The Benchmark

TOMLDecoder now includes benchmarks implemented with [ordo-one/package-benchmark](https://github.com/ordo-one/package-benchmark). 
I plotted the median from the aforementioned benchmark results below.
Each chart includes data points for deserializing the TOML document,
and decoding it on top.
(Unsurprisingly, decoding takes a bit longer.)

The results show 
wall clock time, 
CPU instructions, 
as well as retain count all trending down significantly.

In addition to the before and after,
there's an extra data point measured specifically prior to adopting Swift's `Span`.
More on that later.

<iframe id="benchmark-iframe" src="/assets/2025/12/tomldecoder-0.4.0-benchmark-charts.html" width="100%" height="1200" frameborder="0" style="border: none; display: block; margin: 20px 0; min-height: 1200px;"></iframe>

<script>
window.addEventListener('message', function(event) {
    if (event.data.type === 'resize') {
        const iframe = document.getElementById('benchmark-iframe');
        if (iframe) {
            iframe.style.height = event.data.height + 'px';
            iframe.style.transition = 'none';
        }
    }
});
</script>

## How to make a parser go fast

### Improving data structure and algorithms

... also known as cheating.
Yes, really.

In 0.3.x, `TOMLDecoder` behaves like [JSONSerialization](https://developer.apple.com/documentation/foundation/jsonserialization).
When you ask it to decode TOML data,
with `TOMLDecoder.tomlTable(from:)`
it goes through the entire document,
creates matching container structures within it.
For each TOML table, it creates a `[String: Any]`,
for each TOML array, it creates a `[Any]`.
When a table contains an array,
for example,
a corresponding `["key": [...]]` entry is created to match.
Along the way, the parser also validates the leaf types,
so things like a ill-formed date causes an error to be thrown.
The end result is a `[String: Any]` in which
everything is known to be valid.

A number of things are slow in this process:

* The frequent creation and subsequent usage of intermediary Swift arrays and dictionaries require heap allocations.
* Validating every leaf value takes time.
* Retrieved values are `Any`s, so you have to cast it to the expected type to consume them.


TOMLDecoder 0.4 does away with all of that.

To represent the containers,
and leaf values,
0.4 introduces some light-weight structs,
These structs don't manage the actual memory used to store their contents.
As the parser work through the bytes of a TOML document,
it creates these light weight data types to record the shape of the document,
as well as the byte-offsets of the leaf values.
These intermediary data are stored in a centralized location
to avoid unnecessary heap allocations. 

Here's what I mean by "cheating":
during this phase,
the parser doesn't do much validation of the leaf values.
What it does is more akin to "lexing",
it finds the tokens that could represent a leaf value,
and remembers where they are.
No work is done to actually validate and create the leaf values.

To retrieve any values from the result,
you must state what type is expected:

```swift
// a valid TOML document is always a table at the root level
let serverIP = try TOMLTable(source: tomlString)
	.string(forKey: "ip") // validate this token as a `String`
```

This is an API change.
It delays the validation work,
and helps avoid conversions from `Any`.
If you only need one field,
no validation is necessary on the rest of the leaf values in the entire document.

Swift's decoding APIs ask for typed access:
if your `Codable` type has a `Date` field,
you ask the container for a `Date`,
if the matching value at the spot is of a different type,
an error is thrown.
So the more efficient access pattern benefits the decoding process as well.

### Avoiding bound checks

A major source of slowness in TOMLDecoder 0.3.x is the cost of bound checks in Swift.
The parser holds a reference to the original string,
and hands `Substring`s to small functions to descend on.
A typical piece of the parser might look like this:

```swift
func skipWhitespaces(_ text: inout Substring) {
    let bytes = text.utf8
    var i = bytes.startIndex
    while i < bytes.endIndex {
        if !isWhitespace(bytes[i]) { // bound checks!
            break
        }
        bytes.formIndex(after: &i)
    }
    text = Substring(bytes[i...])
}
```

To avoid out-of-bound access,
Swift inserts logic that checks the validity of the index
for every subscript access of the string's buffer.
A parser does a whole lot of that.
The cost of these bound checks seriously adds up.

Since the release of TOMLDecoder 0.3.0,
Swift has gained a whole set of features that led to the introduction of [Span](https://github.com/swiftlang/swift-evolution/blob/main/proposals/0447-span-access-shared-contiguous-storage.md).
`Span` is built on compile-time lifetime checks.
These checks guarantee the safety when accessing its content.
The same function updated for `Span` looks extremely similar to the original:

```swift
func skipWhitespace(
    bytes: Span<UTF8.CodeUnit>, // aka Span<UInt8>
    remainingBytes: inout Range<Int>,
) {
    var i = remainingBytes.lowerBound
    while i < bytes.count {
        if !isWhitespace(bytes[i]) { break }
        i += 1
    }
    remainingBytes = i ..< remainingBytes.upperBound
}
```

Here,
the subscript access of `bytes` does not incur a bound check!
This created significant performance gains as shown in the benchmark results.

*Here's the kicker*.
The bound checks are eliminated 
because the compiler is confident that the access is safe by construction.
If you make a mistake that would lead to unsafe access,
Swift will refuse to compile your code.
But `Span` is a language feature that requires new language runtime.
You cannot use it on older operating systems.
There's other, older ways to avoid bound checks,
using `UnsafeBufferPointer`s. 
The problem of doing so is that you are responsible for ensuring that the access is safe.
In particular, the point of access must occur in a valid scope for the pointer.
A piece of parser using such API may look like this:

```swift
func skipWhitespace(
    bytes: UnsafeBufferPointer<UTF8.CodeUnit>,
    remainingBytes: inout Range<Int>,
) {
    var i = remainingBytes.lowerBound
    while i < bytes.count {
        if !isWhitespace(bytes[i]) { break }
        i += 1
    }
    remainingBytes = i ..< remainingBytes.upperBound
}
```

But WAIT!  This code using the buffer pointer look extremely similar to the `Span` version!
And if you think carefully,
the requirement for maintaining valid scope for the `UnsafeBufferPointer` is already *enforced* for any `Span`s, syntactically!

Enter [gyb](https://nshipster.com/swift-gyb/). A script that Swift uses to generate repetitive code in the complier.
In TOMLDecoder 0.4,
the parser implementation uses it to generate 2 version of the same set of parsing logic:

```swift
configs = [
    ("Span<UInt8>", "@available(iOS 26, macOS 26, watchOS 26, tvOS 26, visionOS 26, *)"),
    ("UnsafeBufferPointer<UInt8>", "@available(iOS 13, macOS 10.15, watchOS 6, tvOS 13, visionOS 1, *)"),
]
}%
% for byte_type, availability in configs:
${availability}
func parse(bytes: ${byte_type}) throws -> TOMLTable {
	// same code
}
% end
```

... and there's a single place that checks for the OS at runtime:

```swift
let source: String = // TOML string
    if #available(iOS 26, macOS 26, watchOS 26, tvOS 26, visionOS 26, *) {
        let bytes = source.utf8Span.span
        try parse(bytes: bytes)
    } else {
        try source.withUTF8 { try parse(bytes: $0) }
    }
}
```

The beauty here, 
is that the compiler does all the work to ensure the access to the `Span`
as well as the buffer pointer are safe,
because the logic that does the accessing are identical thanks to `gyb`.
## Conclusion

In reality, there are a ton of other optimizations applied in TOMLDecoder 0.4.
For example,
instead of doing dictionary look ups,
looking up things from a TOMLDocument actually involves a linear search.
I know, I know, this goes against what we were taught in CS.
But in modern computers,
and for typical sizes of TOML documents,
a linear search is often faster that computing a hash value,
and the subsequent lookups.

As part of the release, 
the project also gained a bunch of infra improvements.
* It has a [DocC](https://www.swift.org/documentation/docc/)-based [documentation site](https://dduan.github.io/TOMLDecoder/main/documentation/tomldecoder/).
* The entirety of the [official test suite](https://github.com/toml-lang/toml-test) is now programmatically imported as unit tests.
* The source code style is now enforced by [swiftformat](https://github.com/nicklockwood/SwiftFormat).
* Platform checks are more comprehensive and modern on CI.
* Benchmarks are now modernized with [ordo-one/package-benchmark](https://github.com/ordo-one/package-benchmark)

I think of this release as a preparation for a eventual 1.0 release,
which will support the [new deserialization APIs from Swift](https://forums.swift.org/t/the-future-of-serialization-deserialization-apis/78585/171).

Even through I went through some optimizations for speed in this post,
I still have a bunch of ideas I want to try to squeeze out more performance gains.
That's exciting.
