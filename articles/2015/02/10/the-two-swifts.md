# The Two Swifts
2015-02-10T10:50:16-08:00
tag: 

onpatient 2.0 went live in App Store on September 27th 2014, 10 days after iOS
8 was officially released. For the next few months, it was featured for
HealthKit intergration on the App Store front page. In conversations during
that period, people would ask me "what was the experience like?" What they
referred to, of course, was the fact that I wrote that version in 100% Swift.

I have a very different answer to that question now compared to the standard
one I gave back then. I hope to explain to you, my dear readers, how my view
on Swift has evolved since the beginning.

## Swift The Puller

> After you get used to a few nuances, it's Cocoa Touch after all.

People kind of went "huh", and didn't know how to respond to this answer, at
the time when iOS 8 was just out. I think they were expecting horror stories.

Well, they would have been right. One day `UIStoryboardSegue` would have an
`identifier` of type `String!`, the next day it would become a `String?`. And
the incomprehensible error message from Xcode made sure you woundn't figure it
out with ease. This kind of changes would happen to your entire codebase each
time a beta Xcode was released. Compunding with these implementation bugs,
API changes and poor tooling, was the fact that you had to learn a new
programming lanauge. If you and your family's livelihood depends direclty on
the software you put out, avoiding that whole situation was (and is) the
correct *business choice*.

To me, however, all that was just plain *fun*. Within about a week, the
strangeness of having `optional`s went away. It's kind of nice to have the
compiler bark at you when something was unintialized. My old friends never
left: ARC, auto-synthesized properties, blocks, protocols…

As for the beta changes, they were only as often as the Xcode releases. Reading
the release notes and a few careful inspections of the compile errors usually
was enough to help you "fix" everything.

A couple weeks in, Swift *disappeared*. I was just writing a brand new Cocoa
Touch app. My attention was on architecture, user experience and patterns that
works well with varies iOS APIs, business logic … We submitted the app on the
first day it became possible.

Turning back the clock to the ancient days when the world was [blissfully
unaware][Copland 2010 Rev] of Swift's existence. If one wanted to become a Mac
or iOS developer, he/she would have gotten the advice along the lines of
"learning the Objective-C is easy, knowing the system API is harder". Well,
that hasn't changed post-Swift. My experience building software with
UIKit/Cocoa Touch applied 100% in building the 100% Swift App.

Things did change when I went back to Objective-C after a couple months of
writing Swift exclusively, however. The lack of type inference made typing
cumbersome. The worst part is the missing concept of optionality on data
types. *Every* object suddenly were `optional` in my eyes. I exprienced a
sense of insecurity, as if I was driving in a foregn country with simular but
significantly less respected traffic rules.


## Swift The Pusher

> I'm humbled and excited by the future because of what it's teaching me.

By the time I shipped a few major feature updates in Swift, something had
been lurking on the back of my mind: I had left a whole bunch of its features
untouched, yet I seemed to have worked with it to some degree of success. What
gives? The emerging Swift community has shown me the way since. Observe:

Exhibit A, the following Swift function is frequently found in
[libraries][Swiftz] and [books][Functional Book], if not shipping code:

    func curry<A, B, C>(f: (A, B) -> C) -> A -> B -> C {
        return { x in { y in f(x,y)  }  }
    }

Currying is a simple concept. The fact that you can do it in Swift is nothing
to write home about.

What's striking to me, is how *succinct*, or, dare I say, *elegant* this can
be expressed in Swift. In this short piece of code, you see generics, higher
order functions, closures, type inference expressed on top of a strong type
system…to form this basic construct found in functional programming. It's as
if the language has been designed to enable it…

which leads us to exhibit B:

In a job [posting][Swift Job] for Apple's Swift compiler team, the following
qualities are listed for Swift core library engineers (emphasis mine):

> - A passion for making a difference in the lives of other programmers
> - A deep understanding of generic programming principles
> - **Experience with functional programming languages and/or idioms**
> - Experience bridging languages, especially bridging dynamic languages with static ones 
> - Exposure to innovative and/or comprehensive library designs

So in the Swift team's mind, an ideal "customer" of the language, the standard
library authors being the most intimate ones, should be "exploiting" the
language features to practice functional idioms. This detail stuck and
ultimately inspired me to pick up [Functional Programming in Swift][Functional Book]. 
The evidence piled on: [articles][ObjC.io Issue 16], [talks][Swift Conf]
and [Blog][Functional Post 1] [posts][Functional Post 2]. I realized that the
language I've been using for several month has introduced the possiblity to do
what the Haskell/Ocaml/Scala/F# folks do all day long (I won't discuss much
functional programming in this post). The introduction has been gentle and
could be safely ignored, but things like
[automatically generated tests][Fox Testing] and parser combinators got me
hooked.

I believe anyone who write code can benefit from knowing and appreciating the
funtional paradigm. What's unique about Swift is that it's presented on
Apple's huge developer eco-system. That means a huge part of our industry will
benefit as Swift spread futher and futher. In that sense, it really stands out
among all the other languages that enables functional programming.

[Swift Job]: <http://clang-developers.42468.n3.nabble.com/ADVERTISEMENT-open-positions-in-Apple-s-Swift-compiler-team-td4039949.html>
[Swiftz]: <https://github.com/typelift/Swiftz/blob/master/Sources/Swiftz/Curry.swift>
[Functional Book]: <http://www.objc.io/books/>
[Copland 2010 Rev]: <http://arstechnica.com/apple/2010/06/copland-2010-revisited/>
[Reasonableness]: <http://fsharpforfunandprofit.com/posts/is-your-language-unreasonable/>
[Swift Conf]: <http://2014.funswiftconf.com>
[Functional Post 1]: <https://medium.com/swift-programming/2-functional-swift-c98be9533183>
[Functional Post 2]: <http://ijoshsmith.com/2014/11/30/getting-into-functional-programming-with-swift/>
[ObjC.io Issue 16]: <http://www.objc.io/issue-16/>
[Fox Testing]: <https://github.com/jeffh/Fox>
