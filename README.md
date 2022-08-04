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

## Input files

What this program needs as input is essentially a list of notes and rests,
along with their respective durations.
Therefore, ideally, this project would be able to take Standard MIDI files
(SMF) as input, since that is probably the most widespread media format that
conveys that sort of information.
However, parsing MIDI files is by itself a fairly complex task; on top of that,
MIDI files allow any number of notes to be playing simultaneously, which is not
straightforward to implement with this setup.
Therefore, in the end, we've had to settle for a custom text format conveying
note information, like our predecessor, Tempest for Eliza, also used.
(However, hopefully, this format is more readable than the one that project
used...)

The input file format is as follows.

The file must have a number in its first line, and nothing else:
that number is interpreted as the BPM (beats per minute) of the song.
Currently, there's no way to make this value change throughout the song.

On the subsequent lines, there can be any number of notes, separated from
each other by at least one space, with the following case insensitive fields,
from left to right:

- Note name (mandatory): Either `R`, indicating a rest, or `A` through `G`,
according to the English note naming convention.
Non-rest notes can be followed by a `#` (sharp, i.e. raises 1 semitone).
There's no way to indicate flats, so one must use sharps instead,
e.g. A sharp instead of B flat.

- Octave digit (mandatory for non-rest notes):
A single digit indicating the octave number the note belongs to.
The central octave is `4`, and [`A4` is assigned 440Hz frequency][5],
with other notes' frequencies being calculated with the number of semitones
from `A4`, using [twelve-tone equal temperament][6].

- Note duration (optional): The duration of a note, taken as the first letter
of the American English name of the [note value][4]. If this field is omitted,
it defaults to quarter. Currently, there's no way to specify values shorter
than thirty-second.
    - W: whole
    - H: half
    - Q: quarter
    - E: eighth
    - S: sixteenth
    - T: thirty-second

Therefore, `c3`, `a#4h` and `d5e` would all be valid notes under this notation.

It should be noted that, if the same note is played consecutively two or more
times, it will actually sound as a single note with their added duration.
That is, `e3 e3` will not sound as two quarter notes, but as a single half one
(and therefore equivalent to `e3h`). This happens because notes never "fade out"
after being played. This fact can be exploited to achieve ties and dotted notes.
However, if one does want the same notes to sound as individual notes,
the only way around this is placing a small rest between the notes.

[1]: http://www.erikyyy.de/tempest
[2]: https://doc.rust-lang.org/cargo/getting-started/installation.html
[3]: https://www.libsdl.org/index.php
[4]: https://en.wikipedia.org/wiki/Note_value
[5]: https://en.wikipedia.org/wiki/A440_(pitch_standard)
[6]: https://en.wikipedia.org/wiki/12_equal_temperament
