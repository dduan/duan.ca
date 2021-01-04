# Making of a TOML parser
2021-01-03T14:56:10-08:00
tag: Swift, TOML

During the holidays, I spent some time on the parser that underlies [TOMLDecoder][]. The original
implementation targeted TOML's spec version 0.5. A few release-candidates for version 1.0 has come
out since then. So it's a good time to give this project attention.

Back in the 0.5 days, the TOML spec was (well) written in English with a few examples for each
element in the language. The spec document both:

1. how a TOML document should be spelled out (table header should have a `[`, followed by 0 or more
   whitespace, then, a key, 0 or more whitespace, then ']').
2. rules for detecting conflicts in a syntactically correct document (if `a.b` is a table, then it's
   invalid to set its value to be a date in the same document).

My memory of hand-rolling a recursive-descent scanner that conforms to both of the above
requirements was unpleasant. My lesser understanding of the markup language, the vagueness for the
validation rules in the spec, as well as (I think?) poorer skill for writing parsers all contributed
to the unpleasantness. TOML has gained an ABNF for its syntax since then, making it less necessary
to reply on the English descriptions. It's the holidays, maintaining existing code is hard.

So, I decided to have some fun, and re-write the parser from scratch. The rest of this post contains
notable things from the rewrite.

I translated the ABNF with parser combinators. The recent [Pointfree][] video series made me curious
about the performance of this parsing style in Swift. At the end of the rewrite, I added benchmarks
to find out. Here's benchmark for parsing an example TOML document

```
name                    time          std        iterations
-----------------------------------------------------------
example-toml.decoder    454840.000 ns ±   3.63 %       3045
example-toml.combinator 422721.500 ns ±   4.28 %       3266
example-toml.scanner     82232.000 ns ±   9.39 %      16793
example-toml.c            5901.000 ns ±  24.72 %     216140
```

`example-toml.decoder` is time spent on parsing + decoding. `example-toml.scanner` is the time spent
by the old parser. `example-toml.c` is the performance of a C library.

So, the new parser is much much slower! However, I know from debugging through 200+ unit tests that,
the new parser fixed a few serious bugs compared to the old. It also conforms to diffs between
version 0.5 and 1.0 of the TOML spec. Speed alone is not good enough of a reason to throw this work
away! There's another reason the new parser is superior, but I'll talk about it later.

To be fair to parser combinators, I made several decisions that preferred development speed over
runtime speed during development. These were conscious decisions. There were no benchmarks at
time so I didn't want to worry about it pre-maturely. Point is, there are rooms for significant
runtime speed improvements. Perhaps I'll write a follow-up post detailing my journey to make the
parser go fast later; the C library is part of the benchmark for a reason!

I was also inspired by Joe Groff's blog post [Constructing human-grade parsers][]. Instead of
stopping at the first syntax error, a parser (any parser!) should treat the error as part of its
successful output, deal with the erroneous part of the input, and recover from it. This approach
means the portion of the input after the first error gets parsed, and any error it may contain can
be found and reported, too! Joe's post explains this well.

To talk about this further, let's dive into some details.

As mentioned earlier, to validate a TOML document, there are rules for both syntax and semantics to
consider. TOML has a few top-level constructs: table header, array-table header, and key-value
pairs. Each of these alone can be validated purely based on syntax. On a high level, a TOML parser
can do the following:

1. parse a list of top-level constructs
2. iterate over this list, gradually assembly the complete TOML object

Errors could exist from each of these two steps To make the parser *human-grade*, the errors must
not propagate by disrupting the parsing logic. In Swift, this means the code don't throw the
(conceptual) errors, instead, the top-level constructs include error as a possible value:

```swift
enum TopLevel: Equatable {
    case table(Key)
    case arrayTable(Key)
    case keyValue(KeyValuePair)
    case error(Reason)
}
```

Upon evaluating a `TopLevel` value, the parser must not stop if it generates a conflict according to
TOML's requirement. We take note of this error, and move on to consume the next `TopLevel`. Errors
from this step will join the error from `TopLevel.error` at the end for users to see. Therefore, the
second step's code roughly does this:

```swift
var errors = [Error]()

for value in topLevelValues {
    switch value {
    case .error(let reason):
        errors.append(reason)
    default:
        do {
            evaluate(value)
        } catch {
            errors.append(error)
        }
    }
}

if !errors.isEmpty {
    throw UserFacingError(details: errors)
}

// no error! parsing succeeded
```

This code "synchronizes" the errors from both the syntactical, and semantical level. With some
additional effort to make the errors `CustomStringConvertible`, a erroneous TOML document such as

```TOML
a = "Hello
b = 43
[b]
```

makes the parser generate the following error message:

```
Deserialization failure:
    * Value |1, 5| Missing closing character `"` in string
    * Conflict |3, 2| Conflicting value at [b] Existing value is 43
```

The explanation I presented above is simplified by a lot. In reality, the "human-grade" upgrade can
go a lot further. For example, if the definition of `a` has a syntax error, instead of rejecting it,
as long as we can figure out what the intent is, we can pretend it's a good value. That way, any
semantic issues related to `a` can be discovered. Improvements like this can be added to the parser
in many places still.

Anyways, here's where I abruptly stop the story of my new TOML parser. There are a lot of exciting
space for improvment. So this article might be a "part 1". No promises, though.

[TOMLDecoder]: https://github.com/dduan/TOMDecoder
[Pointfree]: https://pointfree.co
[Constructing human-grade parsers]: http://duriansoftware.com/joe/Constructing-human-grade-parsers.html
