# Build And Run iOS Apps In Commmand Line
2015-02-07T20:35:02-08:00
tag: iOS, Objective-C, Swift, DevTool

Xcode is slow. Enough said. What's worse, sometimes I find myself
relying too much on auto-completion with Cocoa Touch, a blessing and a curse!

So I searched for an alternative workflow in command line. The result was
rather confusing: there are posts about using `xctool` or `xcodebuild` to
build Xcode targets, using `ios-sim`, `simctl`  or `instruments` to manage and
manage or launch simulators. Most of the information is out of date.

Eventually though, I was able to piece together an answer for my needs.
That is, given an iOS project set up with Xcode 6, I want to

1. build a target.
2. launch a iOS simulator.
3. install the built .app bundle to the launched simulator.
4. run the installed app.
5. uninstall the app from the simulator.


All in command line, with Xcode *closed*.

Before we proceed to the steps, you need to gather a few pieces of information:

1. the Xcode build scheme of your choice (e.g. "AwesomeApp").
2. your app bundle ID (e.g. "com.awesome.app").
3. name of an existing simulator (e.g. "iPhone 6 Plus"). If you don't want to
   look it up in Xcode GUI, look for it in output of command `xcrun simctl
   list` .

Ready? Here we go.

(These commands should be run in the project folder).

Build the target:

    xcodebuild -scheme AwesomeApp -destination 'platform=iphonesimulator,name=iPhone 6 Plus' -derivedDataPath build


Launch the simulator:

    xcrun instruments -w 'iPhone 6 Plus'


Install the bundle (after simulator is launched and target is built with
previous commands):

    xcrun simctl install booted build/Build/Products/Debug-iphonesimulator/AwesomeApp.app


Launch the app in simulator (after it's installed with the previous command):

    xcrun simctl launch booted com.awesome.app

Uninstall the bundle:

    xcrun simctl uninstall booted com.awesome.app


Quite a few parameters needs to be added for the build step if you have
a comlex project. Please RTFMs. Write some script to automate the steps, if
are a lazy typiest like me.
