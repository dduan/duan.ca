# Taking Screenshots On Linux
2020-06-18T13:22:01-07:00
tags: Linux, macOS

Having used macOS for a number of years, I've formed some habit that I'm never
going to shake. Among them is taking screenshots with `Alt-Shift-3` and
`Alt-Shift-4`, follow up with area or window selection. So, when I bought a PC
and put Linux on it, it's time to mess with it for these habits!

And, it didn't take too long to solve. I've found [maim][]. Writing this down
for mine and maybe, yours, future reference.

To capture the entire screen:

```
bash -c "maim ~/Screenshots/fullscreen_$(date +%s).png"
```

To active area/windows selection for the capture:

```
bash -c "maim -s ~/Screenshots/area_$(date +%s).png"
```

Here's how the latter in action (with keybinds via Ubuntu 20.20 desktop's
Settings app).

![How selecting an area with maim works on Ubuntu](/assets/2020/06/18/area-screenshot-on-ubuntu.gif)

[maim]: https://github.com/naelstrof/maim
