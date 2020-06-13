# StreamLogger
2020-06-13T12:36:55-07:00
tags: Rust, Twitch

After turning off stream on [Twitch][], the first thing I normally do is
exporting the stream video to YouTube, so that the stream has an archive that
survives Twitch's short-ish retain policy.

These videos, perhaps surprisingly, get a few views! It's not a lot by any
measures, but I'm conscientious of the fact that a typical stream archive is
a multi-hour long video with no content curation, no clear schedule, and
it sometimes contains breaks. Needless to say, watching them after-the-fact
requires some (or a lot of) fast-forwarding.

So, last week [on stream][Twitch], I set out to improve the fast-forwarding
experience. YouTube has this feature that lets you jump to specific timestamp
in the video through a URL parameter in the video' link. Further, they generate
this parameter for text in video's description, if the text is in the right
format. This is handy for generating a "table of content" for the video so that
viewers can click the timestamp in the description to jump to the section they
are most interested in.

[StreamLogger][] is a little utility that lets me note down what happened while
I'm streaming. It's kind of like writing a commit message, except the message
describes what happened since the last "commit". These messages, along with
their associated timestamps, will be used by StreamLogger to generate the "table
of content". Using it in command line looks like this:

```bash
# Turn on stream, maybe check signs of being live, etc. Then
slog start # start a new log

# Do stuff, when it comes to a natural conclusion point...
slog -- 'I did stuff'

# Do more stuff...
slog -- 'some other stuff'

# Some time later... end stream
# No action is required to end the stream as far as StreamLogger is concerned.

# Now, to generate the table-of-content
slog stamp -s 1:32
```

That last command outputs

```
0:01:32 I did stuff
1:41:59 some other stuff
```

... which goes to the video's description.

There's a few subtleties in this overall simple tool.

Whenever a message is added, it gets associated with the time at which the
_previous_ message was add. So the act of logging marks both the end of
a chapter and the beginning of the next.

In reality, there's always going to be a gap between the start of the stream and
the time the log is initialized. That's what the `-s 1:32` in the last command
is trying to correct. It tells StreamLogger the length of the gap. Now the
absolute time associated with each event has a relationship with the archive
video.

***

I chose to write this in Rust because I'm going to need the final product on
Linux, macOS, and Windows since I stream on all 3 platforms. (fun fact: I never
built it directly on my PC running Linux. Instead, I simply downloaded the
musl-based build from the GitHub release, which was built by GitHub Actions. It
works beautifully.) Maybe one day I'll add a GUI for it that works across these
platforms, too, so that it's more friendly to wider group of users.

***

The more I use and think about StreamLogger, the more I like it. You can see
me working its entirety in the following archives ;)

Part 1: https://youtu.be/xWRcdaEjir4
Part 2: https://youtu.be/RS-ZMBzu9Dg


[Twitch]: https://twitch.tv/daniel_duan
[StreamLogger]: https://github.com/dduan/StreamLogger
