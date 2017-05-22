---
title: Building Your App In Swift Compatibility Mode
tags: [Swift, Xcode]
date: 2017-05-22 09:04:01-0700
---

It's a few weeks before WWDC, so natually* it's time to try build your Swift
projects in the compiler's compatibility mode!

1. Download and install a snapshot for the next major version [on
   Swift.org](https://swift.org/download/).

    ![download swift toolchain](/assets/2017/05/download-swift-snapshot.png)
    ![install swift toolchain](/assets/2017/05/install-swift-snapshot.png)

2. Choose the newly installed toolchain in Xcode.

    ![choose swift toolchain in Xcode](/assets/2017/05/choose-toolchain.png)

3. Ask the compiler to use compatibility mode. This means using the complier
   flag `-swift-version X`, where "X" is the _current_ major Swift version.

    In project's "Build Settings", value for "Other Swift Flags" should
    contain `-swift-version X`. This could mean setting it in Xcode, in
    `.xcconfig` files you are using and/or in your dependency managers such
    as Cocoapods.

    For example, with Cocoapods, you'll need to add the following in your
    `Podfile` to compile 3rd party libraries in compatibility mode:

        post_install do |installer|
            target.build_configurations.each do |config|
                config.build_settings["OTHER_SWIFT_FLAGS"] = "$(inherited) -swift-version 3"
            end
        end

4. Build your project! This is where things start to get exciting.

   You should expect some warnings. Hopefully they are self-explanatory
   enough. Most of them should correspond to [a swift-evolution
   proposal](https://apple.github.io/swift-evolution/).

   Improvement to the language or compiler usually means some of these
   warnings tell you problems in your code that has been ignored by the
   compiler previous. Fix them today!

   The project should compile successfull in compatibility mode (despite
   warnings). This where you can stop reading. Go celebrate with your
   coworkers, friends, and family!

   Things could go wrong for compiler snapshots, of course. Read on if see
   errors or crashes (whaaaaat 😸).

5. It's time to tell the compiler team about the error or crash you encountered.

   Reduce the error or crash to a state that your are comfortable reporting in
   public. Then go to [bugs.swift.org][https://bugs.swift.org] and file a JIRA
   ticket describing the error or compiler crash.

   During the process of code reduction you may find ways to work around the
   compile error or crash. Make the changes for the workaround and repeate
   steps 4-5. Maybe your project will compile this time.

6. The issue you discovered will be fixed in the official Swift release come
   fall. You've ensured a smooth Swift upgrade for your project and contributed
   to the Swift community 🎉!

---

_Footnote *: source comptibility mode is a thing starting with Swift 4. As new
major version of Swift is released, code written in the previous version should
compile without change in compatibility mode._
