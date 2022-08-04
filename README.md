## What is this?

Some time ago, an amazing project named [Tempest for Eliza][1] was created
to show that old CRT monitors leaked electromagnetic radiation,
some of which was in the usual radio wave frequency, and that,
by displaying specific images on-screen, it's actually possible to manipulate
the electromagnetic waves' shape so that, if a radio tunes to a certain
frequency, it'll play a song.

Nowadays, CRT monitors are a rare sight: everyone uses LCD monitors instead.
However, as it turns out, it is also possible to do the exact same thing
with LCD monitors, albeit rendering slightly different images.
This program is essentially meant to achieve the same as Tempest for Eliza,
but for LCD monitors, hence the name __Tempest LCD__.

## Prerequisites

First and foremost, you'll need a radio that can also handle AM waves
(as opposed to FM only).

Additionally, the key information you'll need to run this program is
your monitor's __horizontal refresh rate__, which is the amount of pixel rows
it is currently rendering per second.
You can obtain this value by simply multiplying the height of the
current resolution by the number of frames per second (refresh rate) currently
in use.
On Linux X11, one can obtain this information with the command `xrandr`,
which marks the current mode with a asterisk. On my computer, this gives:

```
1366x768      60.06*+
```

To the left is the resolution (`width x height`), and to the right is the
number of frames per second currently in use. Therefore,
the horizontal refresh rate of the current mode in my monitor is
`60.06 * 768 ~ 46126`.

**Note**: monitors usually support not only several resolutions, but often
several refresh rates as well. Make sure to take the currently used ones.

## Running

[Make sure you have cargo installed][2], then run

```bash
cargo run -- HORIZONTAL_REFRESH_RATE FILENAME
```

Where `FILENAME` is any file under the `inputs/` folder,
and `HORIZONTAL_REFRESH_RATE` is the value explained above.

On the first run, this command will download all dependencies needed to run this
program. Note, however, that you will also need [SDL2][3] installed.
Consult your Linux distro's documentation for how to properly install it,
but chances are your package manager has it under the name `libsdl2`
or something of that sort.

You may optionally pass the string `cosine` as a third command line argument
(after FILENAME), which causes the program to use sinusoidal waves instead of
square waves. (More on this later.)

[1]: http://www.erikyyy.de/tempest
[2]: https://doc.rust-lang.org/cargo/getting-started/installation.html
[3]: https://www.libsdl.org/index.php
