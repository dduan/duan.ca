# Replying To Old Mailing List Threads
2017-02-07T10:21:19-08:00
tag: Swift Evolution, Email

*One common complains on the Swift Evolution mailing list is about its
inscrutable interface. If you see an inactive thread on the web archive but
haven't subscribed, there seems to be no way to participate or "revive" it
since you never received any of its emails.*

*With a [hint][Erica Email] and some experiments, I've discovered that there is
a way. This post reveals the magic step by step (without commenting on the
merits of email or this solution).*

Like HTTP, Email is a text-based [protocol][Email RFC]. Unlike HTTP, we directly
consume payloads of this protocol and, as a consequence, don't think much about
that very fact.

Like HTTP, each email includes headers hidden by normal email clients. Each
header is essentially two strings, a key and a value, separated by a colon.
Among these headers are `Message-Id` and `In-Reply-To`. When we tell our email
client applications to start a reply, they take the value from `Message-Id`
and use it as value for `In-Reply-To` automatically.

To observe all this in action, we can open some emails with our favorite text
editors and look for these headers. Of course, this require us to know where the
emails exist as files. On macOS, an easy thing to do is to drag from Mail.app to
Finder/Desktop and open the resulting .eml file:

![Open an email in text editors](/assets/2017/02/open-email.gif)

Among the (perhaps overwhelming amount of) headers, we'll find the two fields we
are looking for:

![Email Headers Message-Id and
In-Reply-To](/assets/2017/02/email-headers.png)

… I'll leave the clients' reply behavior regarding these fields for the reader
to verify.

Mailing list software such as GNU Mailman, which [swift.org][] and [llvm.org][]
use to host various mailing lists, associate emails in threads by chaining them
with the headers explained above, among other things. As long as we have
a message's `Message-Id`, we can reply to it "officially" by adding its value to
the `In-Reply-To` header in our email, regardless of whether we have received
that email ourselves previously.

So here are the steps to reply retroactively to a mailing list thread.

1. **find `Message-Id` of the mailing list message we want to reply to.**

   This value is contained in the mailing list web archive's `mailto` link.
   Unfortunately Mail.app doesn't recognize it. It's easy enough to find it
   ourselves though:

   ![Finding Message-Id on mailman
   archive](/assets/2017/02/find-message-id.gif)

   Note the `<`, `>`, `@` characters are percent-quoted. We have to recover the
   id values to the format
   `<alphanumerics-potentially-separated-by-dash@address.com>`.

   Another way to acquire this value is from the gzip'd archive. There they
   just exist as plain text. The downside is you have to dig out the message
   itself first.

2. **Add `In-Reply-To` header to our email.**

   Draft a new email, make its title the same as our email chain's title (this
   is a good idea because lots of email *clients* do use the title to thread
   messages). Set the appropriate recipients, and CCs, including the mailing
   list's address. Now save this email as draft and open it in a text editor as
   we did in our investigations.  Add in the line
   `In-Reply-To: <id-we-found-in-step-one.address.com>`, save it. Then send this
   email (for example, open it with Mail.app and use Message->Send Again in the
   menu).

   Of course, some email client [supports][Thunderbird header support] adding it
   from their GUI.

You'll find the in addition to functioning as an normal message to the
recipients, the mailing list will properly put your message to the original
thread (in the case of swift-evolution, only if your reply is within the same
week).

[Erica Email]: https://lists.swift.org/pipermail/swift-evolution/Week-of-Mon-20170206/031537.html
[Email RFC]: https://tools.ietf.org/html/rfc2822
[swift.org]: https://swift.org
[llvm.org]: http://llvm.org
[Thunderbird header support]: http://www.pixelbeat.org/docs/thunderbird-threading.html
