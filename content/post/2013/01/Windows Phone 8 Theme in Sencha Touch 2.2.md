+++
date = "2013-01-14T13:45:00-06:00"
title = "Windows Phone 8 Theme in Sencha Touch 2.2"
tags = [ "Sencha Touch", "Windows Phone 8" ]
slug = "Windows-Phone-8-Theme-in-Sencha-Touch-22"
+++

Sencha Touch 2.2 Alpha shipped with Windows Phone 8/IE 10 support.
And it's pretty impressive! Just take a look at the starter app under the
original theme and the Windows Phone 8 theme:

The Default Theme:

![Sencha Touch 2.2 default theme](/images/original-theme.png)

WP8 Theme:

![Sencha Touch 2.2 windows phone 8 theme](/images/wp8-theme.png)


And it only takes a minute or two to get to the second screen.
(if you have Sencha Cmd and Sencha Touch SDK 2.2 alpha ready). Here's how.

1.  In SDK folder, generate the app with `sencha generate app Foo path/to/foo`
2.  [Workaround][1] the bug shipped in this alpha version. This won't be
    necessary once the bug has been fixed in the next release
3.  Open `resources/sass/app.scss` in the generated project folder )it should
    look like [this][2] after step 2).
    Replace every appearance of the word *default* to "windows". Then run
    `compass compile resources/sass` in the project root.
4.  Serve the project in a web server (I usually do
    `python -m SimpleHTTPServer`), fire up its url (localhost:800) and...

Boom!

[1]: /2013/01/14/Sencha-Touch-22-Alpha-Sass-Bug-Workaround.html
[2]: https://gist.github.com/4533833 
