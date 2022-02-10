# Building IndexStoreDB on Linux: The Portable Technique
2022-02-09T15:58:41-08:00
tag: Swift,IndexStoreDB

## The problem

As of writing of this article, when you attempt to use Apple's Swift library [IndexStoreDB][] on Linux as
a normal SwiftPM dependency, it won't build successfully:

```fish
❯ cat Package.resolved
{
  "object": {
    "pins": [
      {
        "package": "IndexStoreDB",
        "repositoryURL": "https://github.com/apple/indexstore-db",
        "state": {
          "branch": "swift-5.5.2-RELEASE",
          "revision": "e771994778265c2efe8d33a7ca30adf5f3d2065a",
          "version": null
        }
      }
    ]
  },
  "version": 1
}

❯ swift build > /dev/null

❯ echo $status
1
```

The issue is documented in the build instruction for Linux:

> The C++ code in the index requires `libdispatch`, but unlike Swift code, it cannot find it automatically on Linux. You can work around this by adding a search path manually.

```
$ swift build -Xcxx -I<path_to_swift_toolchain>/usr/lib/swift -Xcxx -I<path_to_swift_toolchain>/usr/lib/swift/Block
```

Okay, so, how would my package build in Linux environments where the Swift toolchain's setup is unknown? How
do we avoid building with one toolchain while mixing in `libdispatch` from another toolchain somewhere? Here's
what I did for my command-line tool [Clue][].

## The solution

The essence of the problem is about the installation location of the Swift toolchain. Our solution makes the
following assumption

1. A Swift toolchain is installed on the file system on Linux (duh!).
2. The toolchain is at least similar to the one distributed on Swift.org. So `libdispatch` is distributed
   alongside the other binaries, in a stable relative directory.

Having the `swift` command available (assumption #1), we can just let it tell us about itself with the
`-print-target-info` flag:

```fish
❯ swift -print-target-info
{
  "compilerVersion": "Swift version 5.5.2 (swift-5.5.2-RELEASE)",
  "target": {
    "triple": "x86_64-unknown-linux-gnu",
    "unversionedTriple": "x86_64-unknown-linux-gnu",
    "moduleTriple": "x86_64-unknown-linux-gnu",
    "compatibilityLibraries": [ ],
    "librariesRequireRPath": false
  },
  "paths": {
    "runtimeLibraryPaths": [
      "/home/dan/.swiftenv/versions/5.5.2/usr/lib/swift/linux"
    ],
    "runtimeLibraryImportPaths": [
      "/home/dan/.swiftenv/versions/5.5.2/usr/lib/swift/linux",
      "/home/dan/.swiftenv/versions/5.5.2/usr/lib/swift/linux/x86_64"
    ],
    "runtimeResourcePath": "/home/dan/.swiftenv/versions/5.5.2/usr/lib/swift"
  }
}
```

Great! We see where the runtime is installed. Now we can invoke assumption #2, that `libdispatch` is at
a relative location to the rest of the runtime. In the output from above, the value for `runtimeResourcePath`
happens to be the parent directory for `libdispatch`'s headers. The `<path_to_swift_toolchain>` value in
[IndexStoreDB][]'s official instruction in this particular setup would be
`/home/dan/.swiftenv/versions/5.5.2`.  So the following command would have worked:

```
swift build -Xcxx -I/home/dan/.swiftenv/versions/5.5.2/usr/lib/swift -Xcxx -I/home/dan/.swiftenv/versions/5.5.2/usr/lib/swift/Block
```

All we need to do is parse this information at build time, and it should work on every Linux setup! Choose
whatever parsing method you like. Here's (more or less) the `Makefile` for [Clue][]:

```make
SHELL = /bin/bash
ifeq ($(shell uname),Darwin)
EXTRA_SWIFT_FLAGS = "--disable-sandbox"
else
SWIFT_TOOLCHAIN = "$(shell swift -print-target-info | grep runtimeResourcePath | cut -f 2 -d ':' | cut -f 2 -d '"')"
EXTRA_SWIFT_FLAGS = -Xcxx -I${SWIFT_TOOLCHAIN} -Xcxx -I${SWIFT_TOOLCHAIN}/Block
endif

define build
	@swift build --configuration $(1) -Xswiftc -warnings-as-errors ${EXTRA_SWIFT_FLAGS}
endef

.PHONY: build
build:
	$(call build,release)

.PHONY: test
test:
	@swift test ${EXTRA_SWIFT_FLAGS}

.PHONY: debug
debug:
	$(call build,debug)
```

1. `make build` / `make test` / `make debug` all work as expected, building IndexStoreDB successfully.
2. As-is, this snippet is project-agnostic. So you can throw it in your SwiftPM project and it should "just
   work".


Alright!

[IndexStoreDB]: https://github.com/apple/indexstore-db
[Clue]: https://github.com/dduan/Clue
