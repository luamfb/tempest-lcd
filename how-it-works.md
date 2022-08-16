# That's cool and all, but how does that even work??

## The overview

There are two hardware components behind this phenomenon: an LCD monitor,
and a radio.

In normal circumstances, a radio only works because there's a station somewhere
broadcasting the waves that it will turn into audio.
However, it just happens that a monitor -- a hardware part meant to emit several
frequencies of visible light -- also emits several frequencies of radio waves.
This isn't so surprising if you consider that visible light and radio waves are
all electromagnetic waves, after all.

Further, not only does the monitor emit radio waves, we can also control
(to some extent) the shape of those waves, through the image we make the monitor
render. Using a specific image allows us to broadcast an AM wave that will make
the radio play a music note: therefore, with a succession of images, we can play
a song.

Of course, this overview is too shallow, and leaves out the most important
questions:
- What does an AM wave that makes a radio play a note look like?
- Why does the monitor emit specific waves when certain images are being
  displayed?
- How can a program know what image corresponds to a given note?

We'll answer that in the following sections.

## Part 1: waves, radios, and modulation

In the end of the day, a radio is meant to turn electromagnetic waves into
sound. But how does it do that? Does a radio simply maintain everything about
the wave it receives -- amplitudes, frequencies, phases -- and just turns it
into audio?

The answer is no. You probably know that to use a radio, we must set it to
"listen" to a given frequency. This frequency never changes, even though the
sound waves the radio plays, often conveying speech or songs, constantly
change their frequency. Plus, if every emitter simply decided to turn sound
into EM waves while maintaining frequency, and they decided to emit them all
at the same time, the waves would certainly interfere with each other.

What does happen is there are two waves involved in the process. One of them is
the **message signal**: the wave we expect the receiving end will hear.
But, in order to transmit the message, we use another wave, called **carrier**,
whose frequency always falls into a fixed narrow range. This way, as long as
each station is given non-overlapping ranges and actually stay in their own
range, the signals will not interfere with each other, and as long as ranges
don't change often, listeners will know which frequency to tune into in order
to hear which station.

The question that remains is how to "insert" the message into the carrier. The
emitter must do this in such a way that is reversible, so that the receiving
radio must be able to separate the carrier from the message, and convert only
the latter into sound. The process of "inserting" a wave into another wave
so that the former can be retrieved later is called **modulation**. Generally,
this process involves making some property of the carrier change according to
the values of the signal at any given time.
As it turns out, there are multiple ways to do that, but the
most commonly used are Amplitude Modulation (AM) and Frequency Modulation (FM).

### Amplitude Modulation

Since this project uses AM, let's take a closer look at it.
If the message is $m(t)$ and the carrier $c(t)$ is a sinusoidal wave
of amplitude $A > 0$ and frequency $F$, that is

$$c(t) = A \sin(2\pi Ft)$$

Then, the modulated signal is given by

$$(A + m(t)) \sin(2\pi Ft)$$

Which means the amplitude of the signal is changed from $A$ to $A + m(t)$.
Note that amplitude modulation only occurs properly if $-A \leq m(t) \leq A$
for all $t$; otherwise, a condition named overmodulation occurs, and the message
cannot be retrieved correctly.

Let's look at the special case where our message is a square wave of frequency
$f$ and the same amplitude as the carrier:

$$m(t) = \begin{cases}
    A & \text{if } \sin(2\pi ft) \geq 0 \\
    -A & \text{otherwise}
\end{cases}$$

This means the modulated signal, which we'll call $s(t)$, will be

$$s(t) = \begin{cases}
    2A \sin(2\pi Ft) & \text{if } \sin(2\pi ft) \geq 0 \\
    0 & \text{otherwise}
\end{cases}$$

Which means the modulated signal will alternate every $1/f$ seconds between
a simple sine wave and no signal at all. This is close to the actual signal
we're making the monitor send with this program -- which leads us to the second
part of this equation.

## Part 2: the LCD Monitor

Now, let's take a look at the LCD monitor's role in this phenomenon.

First and foremost, as [this page][2] will tell you, LCD monitors have a
**grid** or matrix, which directly map to pixels when the monitor is being used
in its preferred (aka native) resolution.
From now on, we'll assume the resolution in use is always the native one.

[This page][3] has an interesting overview of how LCD works, though you don't
have to read it: I'm only using it as a source for the following
(mostly unsurprising) claims:

- While reflective LCD screens do exist, most (if not all) computer monitors
  have a built-in source of electromagnetic radiation.

- Regardless of whether the LCD matrix is passive or active, white pixels emit
  the most intense electromagnetic waves, and black pixels, the least intense,
  with grayscale having proportionate intermediate intensity.
  However, due to light components being implemented as sub-pixels, their
  contribution to signal strength is not so straightforward.

In addition to this, one of the authors of the paper that originated the
Tempest AM project [also wrote a paper][4] on electromagnetic waves leaked by
LCD monitors.
The paper actually explains how to reconstruct text rendered by such monitors
(and, to that end, quite sophisticated hardware is used). However, we can still
learn a few things about LCD monitors from it:

- "these technologies [LCD] update all pixels in a row simultaneously. This
  makes it impractical to separate the contribution of individual pixels in a
  row to the overall light emitted." (page 2)

- "[LCD monitors] still have to continuously refresh the entire image content
  [...] This continuous refresh ensures that the signals on the video interface
  are periodic" (page 3)

Armed with all this information, we can finally figure out what image we want
to give our monitor:

- Sub-pixels are complicated: stick to grayscale.

- Changing colors mid-row is probably a waste of time, as it will do nothing
  but change the average intensity of that row, which we could do by picking
  a different grayscale color for the entire row anyway.

As such, our image should be made of rows of the same grayscale color.

In the specific case of wanting to send a square wave as a message, which means
we'll want our monitor to send a signal like the $s(t)$ presented in the
previous section, we want to alternate between maximum and minimum signal
emission -- meaning the image we want should switch between white and black
pixel rows.

Also, recall that $s(t)$ switches between maximum and minimum levels with
the same frequency $f$ of the message. To find the image we desire to render,
we make the following assumptions:

- Rows are rendered top-to-bottom.

- The time taken to render any row is roughly the same. I don't see why any
  row would be different from any other in how much time it takes to render it.

- If the monitor renders $N$ frames per second and each frame is $h$ pixels
  high, the monitor renders $N h$ rows per second.
  I'm honestly not quite sure of this, since I expected the finished frame to be
  "held" for a bit longer, but this assumption does seem to work in practice.

The product $N h$ is the **horizontal refresh rate** passed as an argument to
the program. Let's call $y$ the Y-coordinate of the current pixel row,
measured in pixels and from the **top** of the screen
(which, conveniently, is precisely what SDL2 does in its coordinate system).
Then, by dividing $y$ by the horizontal refresh rate, we obtain
an estimate of the time $t$ at which we arrived at this row since the frame
started being rendered. Thus, we can finally calculate $\sin(2\pi ft)$ and check
whether it is positive or not, to decide if we should paint the row white or
black. (The actual code doesn't calculate any sine or cosine for this -- but you
may as well read the function `draw_square_wave` and see by yourself!)

## Part 3: the carrier, and how we got away knowing almost nothing about it

You might remember that in part 1, while calculating $s(t)$, we
assumed our carrier was a sine wave. Is this the case here? I have no idea what
sort of radio wave the monitor is emitting, but it certainly isn't a
well-behaved sine wave. However, as it turns out, that doesn't really matter.
We know that whatever this wave looks like, it must be periodic.
Also, a good fellow named Fourier (which is one of those prodigious
mathematicians whose names seem to pop up pretty much everywhere, like Euler,
Laplace and Gauss), found out that almost any periodic function can be written
as a (possibly infinite) sum of sines and cosines, which has been naturally
named [Fourier series][5] after him. That is, for almost any periodic function
$f$,

$$
f(t) = \frac{a_0}{2} + \sum_{n=1}^{\infty} a_n \cos(n t) +
\sum_{n=1}^{\infty} b_n \sin(n t)
$$

For some set of coefficients $\{a_n\}$ and $\{b_n\}$.
Each of these sines and cosines is called a **component** of the signal $f$.

When a radio is tuned to a certain frequency $F$, it effectively "filters out"
or "ignores" any component of the original signal that is not inside a narrow
range centered on $F$. As such, we can completely ignore how the carrier looks
like, and simply look at its components, which are always sinusoidal waves,
and therefore our assumption of having a sinusoidal carrier pretty much always
works.
(Note that sinusoidal waves can be either sines or cosines:
since $\cos(x) = \sin(x + \pi/2)$ for any $x$, the only difference between
those waves is their phase, but phases are irrelevant in this context.)

Now, you might remember that, in order to hear the sound produced here,
one has to slowly change the frequency the radio is tuned to. The reason for
that is we don't really know the values of the coefficients $a_n$ and $b_n$
of each component; more importantly, we do not know the frequency of the
components that have a high enough coefficient that allows the radio to produce
audible sound.
Of course, we could try to obtain the raw signal using a special antenna,
and proceed to make a Fourier analysis to find out the component with the
maximum coefficient whose frequency is also in the range of our radio...
Or, we could just change the frequency until we hear the sound.

However, that begs the question: "what if the signal coming from my monitor
doesn't have **any** component strong enough to produce sound in the frequency
range that my radio can tune to?" Unfortunately, yes, that may happen. The only
solution then is to either use a different radio, or use a different monitor.

[2]: https://computer.howstuffworks.com/monitor6.htm
[3]: https://electronics.howstuffworks.com/lcd.htm
[4]: https://www.cl.cam.ac.uk/~mgk25/pet2004-fpd.pdf
[5]: https://mathworld.wolfram.com/FourierSeries.html
