# Supporting Foundation.Data Without Depending On It
2018-08-18T11:00:41-07:00
tag: Swift, Foundation

While implementing some file I/O APIs in [Pathos][0], I decided reading/writing
file content as `Foundation.Data` is kind of important (can you blame me?). But
Pathos, by accident, does not depend on Swift `Foundation`. Now what?

After browsing the [documentation][1], a pretty good solution emerged: `Data` is
a sequence of bytes! Lets say we hand our users some bytes, they can easily
construct a `Data` from it:

```swift
let content: [UInt8] = try readBytes(fromPath "/tmp/test")
Data(bytes: content)
```

Okay, so this built-in initializer makes `[UInt8]` an acceptable substitute for
returning `Data`. What can we do about about `Data` as input? Well, turns out,
`Data` is a `Collection` of `UInt8`s! So we can accept `Data` indirectly like
so:

```swift
func write<Bytes>(_ bytes: Bytes)
    where Bytes: Collection, Bytes.Element == UInt8
{
    // blah
}
```

User can pass in a `Data` as argument and it just works™.

The only disadvantage of supporting `Data` in these ways is that it requires
your user to discover it either via your excellent documentation, or through
their super good knowledge of `Foundation`.

_Update: this could also [be][2] [slower][3] than using Data directly. Luckily
I'm only doing file I/O here._

But this is pretty nice, regardless.

[0]: https://github.com/dduan/Pathos
[1]: https://developer.apple.com/documentation/foundation/data
[2]: https://mastodon.social/@helge/100573358160444340
[3]: https://twitter.com/daniel_dunbar/status/1030938273047179264
