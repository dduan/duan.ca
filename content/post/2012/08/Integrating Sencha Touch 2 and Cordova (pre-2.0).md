+++
date = "2012-08-20T19:09:34-06:00"
title = "Integrating Sencha Touch 2 and Cordova (pre-2.0)"
tags = [ "Sencha Touch", "Cordova", "PhoneGap", "Mobile Development" ]
slug = "Integrating-Sencha-Touch-2-and-Cordova-pre-20"
+++

"The Goals
-------------

1.  Use Sencha Command to create and package an MVC-structured Sencha Touch 2
    project.

2.  Use Cordova/PhoneGap (1.9) to wrap the project to deliver it in the App
    Store.

3.  Leverage the APIs provided by Cordova/PhoneGap to give Sencha Touch 2
    project more native capabilities.

The Process
----------------------

Goal #1 and #2 can be achieved by [these steps well described by Robert Dougan
][Dougan]. In essence, you need to create a normal Sencha Touch 2 project
using Sencha Command; a Cordova (1.7 in Roberts post, but works fine with 1.9)
project. Then tell Sencha Command to build in the `www` folder, where Cordova
looks for the HTML5 assets to package. Finally, build and deploy the Cordova
project the usual way.

That, of course, is not the end of the story, otherwise you wouldn't be
reading this. When you try to achive goal #3, that is, when you use the
Cordova API in the Sencha Touch project, such as:

        navigator.notification.alert(
            'Winter is coming!',
            noted, // callback function
            'be warned',
            ""Yes M'lord""
        );

Sencha Command will report error and refuse to ""compile"" if you run

        sencha app build package

because of the unknown namespace introduced by Cordova.

One way to workaround this problem is manually replacing the command, which
involves compiling the SASS files, consolidate all Javascript dependencies
into a single file, minify everything and move the result to the `www`
folder.

Doesn't sound fun, does it?

So, here's a trick to put Sencha Command back on track, use

        if (!Ext.os.is.Desktop) {
        }

to wrap around the Cordova API calls, so the previous example becomes

        if (!Ext.os.is.Desktop) {
            navigator.notification.alert(
                // ...
            );
        }

This way, Sencha Command will ignore the conditioned code block since its
running on the desktop environment, therefore compiles like a charm. And the
code will work as intended in emulator/device environment.

You are welcome.


The Bonus
--------------------

Now that you are using Sencha Touch 2 and Cordova API (like a boss!), here's
another tip to improve your workflow. When I'm developing hybrid apps, I spend
the majority of time editing Java(Coffee)script/(S)CSS files. But surely
enough, there will come a period where I have to debug the packaged app in the
emulator. Switching from my text editor (Vim) to XCode and hitting ⌘r
REALLY gets old.

Luckily, Cordova provides some command line alternatives. If
you generate a Cordova project as described in
[Cordova's documentaion][Cordova] like so:

    ./path/to/cordova-ios/bin/create /path/to/project com.example.name Project

There'll be a folder called `cordova` in the newly created project, which won't
be in your project if you create it with XCode with [Robert's method][Dougan].
In `cordova` folder are three lovely Bash scripts that let's you compile the
XCode project (`debug`, which also does the next thing), run the result in  an
ios emulator (`emulate`) and watch the logging information (`log`).

Copy the folder to the path of `YourProject.xcodeproj`, and boom! You can now
quit XCode and run the scripts in there directly from your terminal. (What I like to
do is to use GNU Make to combine Sencha commands with those scripts so that
I could just hit ⌘b in MacVim to make my latest code run in the simulator.)

Happy coding!

[Cordova]:  http://docs.phonegap.com/en/2.0.0/guide_command-line_index.md.html#Command-Line%20Usage ""Cordova Command-Line Documentaion""
[Dougan]:   http://robertdougan.com/posts/packaging-sencha-touch-2-with-phonegap-cordova ""Packaging Sencha Touch 2 with PhoneGap (Cordova)"""