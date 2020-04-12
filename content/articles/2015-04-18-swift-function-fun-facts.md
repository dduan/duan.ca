+++
title = "Swift Function Fun Facts"
date = 2015-04-18T16:20:16.078786-07:00
[taxonomies]
tags = ["Swift"]
+++

You love Swift. You want to write a class that does HTTP, it might have
methods like the following:

```swift
func get(URLString:String, params:[String:AnyObject],
    headers:[String:String])
func post(URLString:String, params:[String:AnyObject],
    headers:[String:String])
func put(URLString:String, params:[String:AnyObject],
    headers:[String:String])
// and more for HEAD, OPTIONS …

```

But you don't want to force your user to supply all arguments each time. You
know that Swift supports default arguments, so you added some. Take GET as an
example:

```swift
func get(URLString:String, params:[String:AnyObject]=[:],
    headers:[String:String]=[:])

```

Now users can do things like

{% highlight swift %}
get("http://github.com")
get("http://httpbin.org/get", headers:["Answer":42])
{% endhighlight %}

That's flexible! Woohoo!

After you thought about implementing these, though, you realize that
`HTTPMethod` is merely a property on `NSURLRequest`. In other words, all of
the previous methods can share the same implementation. In honor of the DRY
principle, you write a function that accepts the method as an arguments and
the previous functions each forwards the arguments to this function:

```swift
func impl(method:String, URLString:String,
    params:[String:AnyObject],
    headers:[String:String])
{
    // …
}

func get(URLString:String, params:[String:AnyObject]=[:],
    headers:[String:String]=[:])
{
    impl("GET", URLString:URLString, params:params,
        headers:headers)
}

func post(URLString:String, params:[String:AnyObject]=[:],
    headers:[String:String]=[:])
{
    impl("POST", URLString:URLString, params:params,
        headers:headers)
}
```

This seems like a sensible solution. Except that later you realize that there
needs to be more parameters for each function, so in the end, each function
looks like this:

```swift
func post(
    URLString             : String,
    params                : [String:AnyObject]       = [:],
    json                  : [String:AnyObject]?      = nil,
    headers               : [String:AnyObject]       = [:],
    auth                  : (String,String)?         = nil,
    allowRedirects        : Bool                     = true,
    requestBody           : NSData?                  = nil,
    URLQuery              : String?                  = nil,
    asyncCompletionHandler: ((HTTPResult!) -> Void)? = nil
    ) -> HTTPResult {
    return impl(
        "POST",
        URLString             : URLString,
        params                : params,
        json                  : json,
        headers               : headers,
        auth                  : auth,
        data                  : requestBody,
        URLQuery              : URLQuery,
        redirects             : allowRedirects,
        asyncCompletionHandler: asyncCompletionHandler
    )
}
```

Remembering that your goal is to respect DRY, and there are now giant blocks
of code that all look the same except that first argument to `impl()`, you
became determined to find a better alternative.

Well, why not give currying a try? This example of currying with Swift comes
to your mind:

{% highlight swift %}
func add(a:Int)(b:Int) -> Int { return a + b }
let add3 = add(3)
add3(b:2) // 5
{% endhighlight %}

If we apply this technique and treat `method` in `impl()` as `a` in the
example, we would get:

```swift
func impl(method:String)(
    URLString:String,
    params:[String:AnyObject],
    headers:[String:String],
    …)
{
    // …
}
let get = impl("GET")
let post = impl("POST")
```

right?  However, you are forcing users to supply each argument again. To make
things worse, the number of arguments is a lot larger.

Hmm, but that's a solved problem, just add default values to `impl()`'s
parameters:

```swift
func impl(method:String)(
    URLString:String,
    params:[String:AnyObject] = [:],
    headers:[String:String] = [:],
    …)
{
    // …
}
```

Ta-da! Wait a minute, Xcode now refuse to compile you code! `Default argument
is only permitted for a non-curried function parameter`, it saids.

Stubborn as you are, you decide that perhaps the Swift team hasn't got around
to implementing this feature for curry syntax yet. Functions are first-class
citizens! Surely if you return a function with default arguments…?

```swift
func methodFactory(method:String)
    -> (params:[String:AnyObject] = [:],
        headers:[String:String] = [:], …)
    -> Void
{
    return {(params, headers, …) in
        impl(method, params:params, headers:headers, …)
    }
}

let get = methodFactory("GET")
let post = methodFactory("POST")
```

Turns out, this manual form of currying only works when default arguments
aren't involved!

Now, you hate Swift.

(Just to be clear, I don't really hate Swift as in, uh, *hate* Swift. Judgning
from some comments, I might have failed to convey the lightheartedness that
I felt writing this up. It's really like saying to a friend "I hate you" after
he/she pulls a prank on you.)
