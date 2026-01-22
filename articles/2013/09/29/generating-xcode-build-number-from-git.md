# Generating Xcode Build Number From Git
2013-09-29T15:50:50-06:00
tag: Xcode, Git, Cocoa Touch

The build version number in an Xcode project (CFBundleVersion in Info.plist)
must be a monotonically increasing string for each archive submitted to the
App Store. Neglection of this requirement will result in an error during the
binary upload. Some automation will help avoiding this issue.

First, we want to generate this version number from our version (*duh*)
control system (VCS) each time our target gets built. My VCS of choice is Git, 
users of other systems just need to get a increasing number from their code
history. On a \*nix system, this command will count the number of commits on
'develop' branch up until HEAD:

        git rev-list develop | wc -l

This is a very good candidate for our build number, for longer history
generally correlates to later builds.

Next, we make Xcode automatically run this command and use its result as the
build number.

Go to project navigator, select your build target under the project icon,
click *Build Phases*, select *Editor→Add Build Phase→Add Run Script Build
Phase* in the menu. Remove the content in editor of the new *Run Script*
phase and replace it with:

    revnum=`git rev-list develop | wc -l`
    echo "#define BUILD_NUMBER $revnum" > InfoPlist.h
    touch InfoPlist.h

Go to build settings, change value of *Preprocess Info.plist File* to "YES".
Add "InfoPlist.h" to *Info.plist Preprocessor Prefix File→release*.

We ask Xcode to run our command and save its result as a "#define" in a header
file when it builds the project. This is done so that we can replace the
"hardcoded" build number with the name of the constant:

Open Info.plist, Double click the value of *Bundle Version* and replace it with

    BUILD_NUMBER

There's only one issue left: `BUILD_NUMBER` is now saved in *InfoPlist.h*. Its
value comes from our commit history. So we want to exclude this piece of
information as part of our commit history (I'll leave the reason as an exercise
for the reader). Ignore this file by adding "InfoPlist.h" to *.gitignore* (or
that of your other VCS).

To recap, when you build the project now, Xcode will find out how many commits
are in the history, define it in a header file as `BUILD_NUMBER`, which gets
used as the build number. As long as you keep with with version control, the
build number problem goes away.
