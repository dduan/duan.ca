# Sencha Touch 2 and PhoneGap integration
2013-05-28T15:24:30-06:00
tag: Sencha Touch, PhoneGap, Cordova

As one of my pet Sencha Touch project gets close to finish, I started
looking into distribute it as native apps with Phonegap/Cordova.

One of the concerns in do so is the 'deviceready' event provided by Phonegap,
according to the documentation:

> This is a very important event that every Cordova application should use.  
> ...

> The Cordova deviceready event fires once Cordova has fully loaded. 
> After the device has fired, you can safely make calls to Cordova function.
>
> Typically, you will want to attach an event listener with 
> document.addEventListener once the HTML document's DOM has loaded.

In particular, the nice Sencha Touch microloader complicate the matter by
being the sole Javascript file that's supposed to be included in `index.html`
and is in charge of starting the actual code of our apps. Yet we need the
starting point of the code be a response to the `deviceready` event.

After some googling, I found that most information on this matter is either
inaccurate, incomplete or outdated, that is until I found [this answer][1] by
[Robert Dougan][2] on StackOverflow:

> Sencha Touch 2 will listen to that event and call your onReady/launch methods
> - therefore if you try listening to them in the launch method,
> it has already been fired.
>
> Just put your logic inside the launch method in your application.

To verify this claim, I dug into `sencha-touch-debug.js` distributed with
Sencha Touch 2.2 and found the following code:

    if (Ext.browser.is.PhoneGap && !Ext.os.is.Desktop) {
        if (!Ext.readyListenerAttached) {
            Ext.readyListenerAttached = true;
            document.addEventListener('deviceready', triggerFn, false);
        }
    }

It appears that the `deviceready` event is taken into account here as long as
`Ext.browser.is.PhoneGap` is true in a mobile browser envronment, which, in the
same source code, means:


    if (typeof window.PhoneGap != 'undefined' ||
        typeof window.Cordova != 'undefined'  ||
        typeof window.cordova != 'undefined') {
        isWebView = true;
        this.setFlag('PhoneGap');
    }


Here, the global variable PhoneGap, cordova or Cordova needs to be defined to
satisfy Sencha Touch 2's expectation of PhoneGap environment. Those globals
are defined in the `cordova-x.y.x.js` file included in the PhoneGap/Cordova
project files.

So what needs to be done for the integration is simple (if not clear):

include `cordova-x.y.x.js` in the js section of `app.json` project file so that 
the microloader knows to load it up early:

    "js": [
        {
            "path": "path/to/cordova-x.y.z.js",
        },
        {
            "path": "touch/sencha-touch.js",
            "x-bootstrap": true
        },
        {
            "path": "app.js",
            "bundle": true, 
            "update": "delta"
        }
    ],

Run `sencha app build package` and drop the files it produces to the `www`
folder in the PhoneGap project.

Compile, ship.

[1]: <http://stackoverflow.com/a/10457158/243798>
[2]: <http://dougan.me>
