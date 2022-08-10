## That's cool and all, but how does that even work??

### The overview

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

### Part 1: waves, radios, and modulation

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
