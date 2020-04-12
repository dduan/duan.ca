+++
title = "Distribution Tests"
date = 2016-10-10T10:34:49-07:00
[taxonomies]
tags = ["Swift", "Cocoapods", "Carthage", "Swift Package Manager", "Make"]
+++

There are several ways to build dependencies for a Swift 3 project:
Swift Package Manager, Cocoapods, Carthage, etc. Many have an opinion on which
is the best choice for their projects. As a library author who want to help as
many people as possible, however, they can't ignore any of them.

I sometimes question the sanity of that conclusion: instead of becoming an
export in each package manager, I find myself a novice of all. Did I break
support for any of them with this project change? Is it still working on
a particular platform, say, tvOS? Can I *really* know?

The only way to *really* know is to verify yourself: both Cocoapods and
Carthage support four platforms -- iOS, macOS, watchOS, tvOS; Swift Package
Manager only works on Mac or Linux. So that's 2*4+1=9 targets. All these
targets need is have the library in question fetched and imported. Every Swift
library author should have such verification before publishing updates of
their project.

The steps to verify can be triggered with commands: fetch and build
dependencies, build Xcode/Swift project. To automate the process, put these
commands in a script or a Makefile. But wait, there's more! One shouldn't have
to create these dummy projects every time they create a new library. If all
these projects do is importing a library and attempt to build, they should
work for *any* library. The config in `Package.swift`/`Cocoapods`/`Cartfile`
and the `import` statements just needs some strings replaced: name of the next
library, URL for its git repository, etc. And that's a scriptable process as
well!

To recap, one could, in theory, copy in some dummy projects, run a command to
inject information about a new library, run another command to build all these
project, verifying that support for those package managers remain functional.

In reality, I have created [DistributionTests][DistributionTests] and put it
on [Github][DistributionTests] 😉.

The script `customize` requires 3 pieces of information of the library: its
import name, its git repository URL and a major version number. The assumption
here is the library generates uniformly named artifacts: the file name for
`.framework` and the name users use to import it are the same. Testing
distribution of a library is as simple as:

1. clone the project.
2. customize the project with `customize`.
3. run `make`.

If you do step 1 and 2, include the projects in library's repository, then
only step 3 is necessary! This makes testing distribution methods trivial on
a continuous integration server.

Go forth and create fearlessly!

[DistributionTests]: https://github.com/dduan/DistributionTests
