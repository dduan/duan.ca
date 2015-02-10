+++
date = "2014-05-02T10:25:00-06:00"
title = "One Weird Trick To Make Vim Go Faster On Your Mac"
tags = [ "iTerm2", "Terminal", "OS X", "Vim" ]
slug = "One-Weird-Trick-To-Make-Vim-Go-Faster-On-Your-Mac"
+++

I noticed something strange today.

While playing with Ubuntu on a VirtualBox hosted by OS X Mavericks, Vim
_seems_ much faster than it being in iTerms2. How could that be? So I took the
following steps to test things out:

*   Installed exact configurations with Vundle on the VM.
*   Vim in Terminal.app
*   MacVim with GUI.

Nope, Vim is still more responsive on Ubuntu. In fact, text input seem more
responsive on this VM than the host OS in general! I thought I came to the
light switching to OS X after using Linux as desktop for years, and now this?
_[WTF][WTF]?_

Defeat and confused, I went to the Internet, and found [something close to an
answer][close answer]. Under _System Preferences-Keyboard_, drag the two slide
widget ("Key Repeat" and "Deley Until Repeat") to the right most. Suddenly,
Vim become faster!

Turns out, key repeating is very important for Vim users.

[WTF]: http://developer.android.com/reference/android/util/Log.html#wtf(java.lang.String,%20java.lang.Throwable)
[close answer]: http://stackoverflow.com/questions/4489885/how-can-i-increase-cursor-speed-in-terminal
