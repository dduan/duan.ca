# Making An Open-Source Swift Project
2017-12-17T21:24:43-08:00
tag: Swift, YouTube, Let's Build, Xcode, CocoaPods, Carthage, SwiftLint, iOS, tvOS, macOS, watchOS

This video shows what it takes to open source a Swift project. Starting from some code in a playground,
I created a framework that supports distribution via Swift Package Manager, CocoaPods, Xcode project and
Carthage. The framework can be used across iOS, macOS, watchOS, tvOS and Linux. In addition, we also added
SwiftLint to the project, added a Makefile, put everything on GitHub and set up continuous integration via
Travis. README, COC, LICENSE and CHANGELOG are stubbed in as well.

<div class="video-container">
    <iframe src="https://www.youtube.com/embed/pA0T1CdqMt8" frameborder="0" gesture="media" allow="encrypted-media" allowfullscreen></iframe>
</div>

This is not at all a comprehensive list of things a good Open Source project should have. We are still missing
the version tag, documentation generation, automatic Carthage binary generation, test coverage...

Anyways, here's some stuff mentioned in the video:

* Sample project from the video: <https://github.com/dduan/BitArray>
* Video of me writing the code in this project: <https://www.youtube.com/watch?v=-k_jrIoD56k>
* Swift Package Manager: <https://swift.org/package-manager/>
* CocoaPods: <https://cocoapods.org>
* Carthage: <https://github.com/Carthage/Carthage>
* Xcconfigs for universal framework: <https://github.com/mrackwitz/xcconfigs>
* SwiftLint: <https://github.com/realm/SwiftLint>
* Travis CI: <https://travis-ci.org>
