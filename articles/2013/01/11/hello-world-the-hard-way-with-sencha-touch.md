# "Hello, World!" The Hard Way with Sencha Touch
2013-01-11T00:00:00-07:00
tag: Sencha Touch, Mobile Development

When I first got into the amazing Sencha Touch HTML5 framwork, it came across
as a compilation of visual components that looks mobile, [easily themable](http://compass-style.org),
leverage the latest HTML5 technology to be efficient and, best
of all, are created within the Javascript code as oppose to being shoved
in to an HTML file and demand DOM tinkering later.

But a closer look would reveal a lot more goodies beyond those handy
components in Sencha Touch. It offers [a class system](https://web.archive.org/web/20130117172701/http://docs.sencha.com/touch/2-1/#!/guide/class_system), a [M](https://web.archive.org/web/20130120111258/http://www.sencha.com/learn/architecting-your-app-in-ext-js-4-part-1/)[V](https://web.archive.org/web/20130106185309/http://www.sencha.com/learn/architecting-your-app-in-ext-js-4-part-2)[C](https://web.archive.org/web/20130124092547/http://www.sencha.com/learn/architecting-your-app-in-ext-js-4-part-3/)[pattern](https://web.archive.org/web/20130109001131/http://www.sencha.com/blog/architecting-your-app-with-sencha-touch-2-mvc/), tools that handles code dependency, compression and native
packaging, etc. Albeit daunting, learning and embracing all of those offerings
makes a quite enjoyable coding experience and rewards me with development
effieciency overall.

Sometimes though, I need complete control of a visual component that doesn’t
exist in the framework. How to make this work with everything mentioned above
is implied in various tutorials posted by the Sencha team, but I couldn’t find
a clear illustration of that, which is why I decided to write one here.

Here’s our goal: make a component that displays data from a model with our own
custom HTML; manage it along with some provided components from the framework
and follow the MVC pattern.

I assume you have the basics set up. I’m using Sencha Cmd 3\.0\.0\.250, Sencha
Touch SDK 2\.1\.0 and OS X Mountain Lion as of this writing.

Let’s start by generating a MVC\-ready skeleton project called HelloWorld. Go to
the SDK’s folder and type:

```
    sencha generate app HelloWorld ~/helloworld

```
replace the last parameter with the path you would like for the project files
to stay. We’ll be working in this folder from now on.

In the skeleton project, a main view was created under `app/view/Main.js` and
declared as dependency for `app.js`. An instance of it is created when the app
finished loading. We’ll keep this setup as our main view. Let’s reduce`app/view/Main.js` to a simplest possble form for our purposes:

```
    Ext.define('HelloWorld.view.Main', {
      extend: 'Ext.Container',
      config: {
        items: [ { xtype: 'helloview' } ]
      }
    });

```
Following the convention, we name the class in corrispondance with its
filepath within the `app` folder (‘view/Main.js’ \=\> ‘view.Main’). All
classes created with `Ext.define` should follow this convention so that Sencha
tools can relate code dependencies to file structure and do its magic for us.
We’ll circle back to this.

Our main view will be a plain container and has a `helloview` in it. An`Ext.Container` has the ability to … contain stuff. Specifically, it can
organize `Ext.Component`s visually. `helloview` will be that component. Let’s
define it next.

Create the file `app/view/HelloView.js` as the following:

```
    Ext.define('HelloWorld.view.HelloView', {
      extend: 'Ext.Component',
      xtype: 'helloview',
      config: {
        tpl: '<div class="greeting">Hello, {name}!</div>'
      }
    });

```
As promised, `HelloWorld.view.HelloView` is a `Ext.Component`. We declare its`xtype` to be the one we used in the main view. The really interesting part is
its `tpl` configuration. This is where our customization integrates with the
rest of the Sencha Touch framework, so it’s worth dive into a bit deeper.

(*Caution*: `tpl` can not be mixed with `items`, if you want standard components
mixed in with yours, make them share a container and arrange a proper layout
there.)

Take a look at the official [documentation for `tpl`](https://web.archive.org/web/20130117172701/http://docs.sencha.com/touch/2-1/#!/api/Ext.Component-cfg-tpl). It accepts an[`Ext.XTemplate`](https://web.archive.org/web/20130117172701/http://docs.sencha.com/touch/2-1/#!/api/Ext.XTemplate), which is basically free\-range HTML plus some syntax to
insert data provided to the component. Apply all your HTML skills here! For
illustration purposes, we only throw in a basic `<div>`.

The `{name}` part will be replaced by the actual data, which every`Ext.Component` has as a configuration option by default. Being such option
means two things:

```
1. You can specify its value in the class definition, as we did for `tpl`.
2. It gets a "getter/setter" that let you query/change its value at anytime
    through the instance's existence.

```
We’ll use the setter for `data` – `setData()` to populate this field in the
template later. `setData()` accept one raw Javascript object and make its
properties accessible to the template.

Next, let’s make a simplistic model in `app/model/Greetee.js`:

```
    Ext.define('HelloWorld.model.Greetee', {
      extend: 'Ext.data.Model',
      config: {
        fields: ['name']
      }
    });

```
A model, with a field ‘name’. Hey, that’s `as simple as possible, but no
simpler`, Einstein would endorse it!

We won’t use any proxy or store in conjunction with the model because we
only need to show how a customized view works within the Sencha Touch MVC
pattern. Speaking of which, a controller does just that. So to tie everything
togeter, here’s `app/controller/Main.js`:

```
    Ext.define('HelloWorld.controller.Main', {
      extend: 'Ext.app.Controller',
      config: {
        models: ['Greetee'],
        views: ['HelloView'],
        refs: {
          helloView: '.helloview'
        }
      },
      launch: function() {
        var m = Ext.create('HelloWorld.model.Greetee', { name: 'World' });
        return this.getHelloView().setData(m.getData());
      }
    });

```
To get reference to the view components, Sencha Touch provide `refs` in
controllers, through which we map `hello` to our customized component’s xtype.
There are more details about this in the [offical documentation](https://web.archive.org/web/20130117172701/http://docs.sencha.com/touch/2-1/#!/guide/controllers). We now
can use `getHelloView()` in other methods. `launch()` gets invoked after
everything gets loaded, and we tie the model and the view together here.

Again, a little imagination might help. By that I mean the data source of the
model could be from a RESTful network API, a picture from a phone camera via
Phonegap, a record from browser’s localstorage, etc. We simply created one in
memory for illustration.

We use `getHelloView()` to get reference to our `helloview` instance, then use
its `setData()` to populate its template field. But we can’t pass in the model
object directly (as mentioned above, a raw Javascript object is needed), so
we convert it with its `getData()` method.

Finally, we specify the `models` and `views` involved with this controller in`config` so that the files of these classes gets loaded properly. For the same
purpose, we need to open `app.js` and add the following line into the object
passed to `Ext.application()` (which is a controller too):

```
    controllers: ['Main'],

```
This makes the framework aware of our `HelloWorld.controller.Main`. It’s
unnecessary to use the full class name becaue we followed the naming
convention.

At this point, our code is complete. Go to the project folder in terminal
and fire up a web server:

```
    python -m SimpleHTTPServer

```
Open <http://localhost:8000> in your browser and take a gander!

A screenshot of this app would be an overkill, if you follow along correctly,
the phrase “Hello, World!” will show, and that’s all.

So this long\-winded excercise results in not much. But I hope you won’t find
it pointless. When I try to construct my app UI with Sencha Touch, I first try
my best to make the Senche component work with the design. When breaking out
and customize is inevitable, I try to stay within the framwork as much as
possible to make the most out of it. What’s described in this article is a
common way to do that.
