# The Lazy Programmer's Guide to Private Mastodon
2022-12-10T18:21:11-08:00
tag: Mastodon, Twitter, Digital Ocean, DNS

As of today, my identity in the "fediverse" has become `@daniel@duan.ca`. This post was originally published
on `duan.ca`. Pretty neat, eh? I achieved this by hosting a private instance of the open-source Mastodon
project. Here's my experience setting that up, and why it seemed harder than it is.

## What exactly did I achieve?

I own the domain `duan.ca`, which serves as the root of an existing website. I set up an functioning
instance of Mastodon with a single user `@daniel`. This user is discoverable in other Mastodon instances as
`@daniel@duan.ca`.

## How did I do it?

As someone who barely touch server-side programming, running a Mastodon instance seemed very daunting to me.
What database to use? How? What's the best way to host a Rails app? I need a email server? Where to host the
media files I post?

There are a lot of step-by-step set-up tutorials on the internet. This post is, definitively, not one of
those. But having gone through the process of [setting up an instance on DigitalOcean][], I end up with a few tips
and tricks to share! Let's through them:

1. If you have a Digital Ocean account, setting a server up is as simple as searching up "Mastodon" in their
   market place as a step in creating your Droplet.
2. You _do not_ need an email server for a single-user instance. Think about it, you have root access to the
   database, what do you need email for? Verify your own identity? Spam yourself?
3. That said, Sendgrid is free for low-volume emails.
4. Ok, you ssh into your Droplet, the first thing you see is a guided CLI experience for setting things up.
   Don't go through it just yet! For me, my root domain `duan.ca` hosts an entirely different site. So I have
   to pick a subdomain to host Mastodon. Make sure you set up an A record in your website's DNS for your
   chosen subdomain. If this record isn't set up, the set up will fail when it tries to get an certificate
   from Let's Encrypt.
5. Unless you want to buy a new DNS service from Digital Ocean, your A record should be set with your existing
   DNS server provider! I needed a few minutes to un-confuse myself about this fact. Digital Ocean's
   documentation made it really hard to discover this fact.
6. The only thing you need to get right is your subdomain, and DNS record for it. In the rest of the guided
   setup experience, you can make mistakes, and fix them later.
7. I used an S3 bucket for media. S3's UX is kind of terrible, so if you are doing it for the first time, good
   luck.
8. Instead of "admin", choose your Mastodon username right then, and there. This user does not have special
   privilege for the site anyways, despite what the default name suggests. After this step, the guide will
   provide you with a password, take note of that if you want to log in with this user later :)
9. Again, you could figure out how to get an STMP server to send emails on your Mastodon instance's behalf.
   But you are the only user, right? So you can just make some value up for all things SMTP/email related. At
   the end, when the guide asks whether to send a test Email, choose "no".
10. The Mastodon code is located at `/home/mastodon/live`. Most of the information is stored in
    `.env.production` in that directory. This is where you can change your mind/fix your mistake from the
    guided setup. You can use Vim instead of Nano. (Although if you like Vim, you'd probably find out
    anyways).
11. Mastodon ships with an admin [CLI tool][] `tootctl`. It's located in `bin/tootctl`.
12. Before attempting to run this tool, switch to user `mastodon` by running `su mastodon`. Otherwise, you'd
    be confused as to why you need to install Ruby stuff, and why you can't connect to the Postgres database.
13. If the guide didn't tell you an error, and your values in `.env.production` are all right, you will be
    able to type your subdomain in a browser and see Mastodon! Log in with your username, and password from
    the guide. Upload a profile photo to test whether your media service is set up correctly.
14. DO NOT DO THIS YET! At this point, if you go to an existing Mastodon instance, and look yourself up! If
    your subdomain is `social.duan.ca`, username `daniel`, you should be discoverable as
    `@daniel@social.duan.ca` on a site such as [mastodon.social][] now. But if you do this, that existing site
    may get confused and permanently consider that as your fediverse identity. So don't do that! Finish the
    next step.
15. To use your root domain instead of this subdomain, you 301 redirect `/.well-known/host-meta*` from root to
    the real location. [this article][Redirect] explains it very well. Make sure to both set up the URL
    redirect, as well as `LOCAL_DOMAIN`, and `WEB_DOMAIN` in `.env.production`.
16. Make your user the owner of the site (RTFM). You'll see additional, site-wide settings afterwards. Make
    sure to disable signup, either in the GUI, or with `tootctl`.

That it! These may seem like a lot of tips, but I think, as a smart programmer, you can eventually figure all
of these things out pretty easily. I put them down here because, had I known these, I would've saved a bunch
of times in the end.

[Redirect]: https://masto.host/mastodon-usernames-different-from-the-domain-used-for-installation/
[CLI tool]: https://docs.joinmastodon.org/admin/tootctl/
[setting up an instance on DigitalOcean]: https://marketplace.digitalocean.com/apps/mastodon
[mastodon.social]: https://mastodon.social

## Why?

Why spend the time, and money, to go through all of this?

This is a really good question. I [wished][https://duan.ca/2022/11/20/the-social-graph/] less people would
leave Twitter. I host my personal site/blog on my own domain, as opposed to using a Medium site or something.
So, if I have to "federate" my microblogs, why shouldn't I try to gain more independence? How many of these
mastodon instances can do better than Twitter, in terms of site maintenance, and content moderation? Instinct
tells me, something between self-hosting, and Twitter, might be the right balance. But I'm here for the long
haul, and, if self-hosting proves to be the wrong direction, I'll pivot.

Until then, this has been fun.
