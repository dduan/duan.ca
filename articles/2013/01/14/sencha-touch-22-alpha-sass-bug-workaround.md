# Sencha Touch 2.2 Alpha Sass Bug Workaround
2013-01-14T13:47:00-06:00
tag: Sencha Touch

Sencha [released][1] a new version Sencha Touch with Windows Phone 8 support.
But since it's an alpha, there are a few more things to do than what the 
[release note][2] says to get it working.

One thing I've noticed is that when you generate a new app with

    sencha generate app Foo path/to/foo

and make the changes to `resources/sass/app.scss` according to the release
note, `compass compile path/to/foo/resources/sass` fails complaining a font
file is missing:

    File not found or cannot be read: path/to/foo/resources/sass/stylesheets/fonts/pictos/pictos-web.woff

To fix this, copy the needed fonts to where it supposed to be:

    cd path/to/foo
    mkdir -p resources/sass/stylesheets/fonts/pictos
    cp touch/resources/themes/fonts/pictos/pictos-web.* resources/sass/stylesheets/fonts/pictos/

The next error you'll see from `compass compile` is caused by the name changes
to a few sass files in the framework. Long-story short, you need to change
`resources/sass/app.scss` to the following:


<script src="https://gist.github.com/4533833.js"></script>

and `compass` should be happy from there.

While you are at it, why not checkout the magical Windows Phone 8 theme
included in Sencha Touch 2.2 alpha.

[1]: <http://cdn.sencha.com/touch/alpha/touch-2.2.0-alpha.zip>
[2]: <http://cdn.sencha.com/touch/alpha/2.2.0.52/release-notes.html>
