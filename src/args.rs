// Copyright (C) 2022 Luana Martins Barbosa
//
// This file is part of tempest-lcd.
// tempest-lcd is free software, released under the
// GNU Public License, version 2 only.
// See COPYING.txt.

#[derive(clap::Parser)]
pub struct Args {
    #[clap(verbatim_doc_comment)]
    /// The product of the native resolution's width
    /// and the monitor's refresh rate (in Hz),
    /// as explained in README.md.
    pub horiz_refresh_rate: f64,

    #[clap(verbatim_doc_comment)]
    /// The file to be played.
    /// If using --midi, must be a MIDI file.
    /// If not using --midi, must be a text file
    /// with the format explained in README.md.
    pub filename: String,

    /// Use cosine waves instead of square waves as signal.
    #[arg(long)]
    pub cosine: bool,

    /// Use experimental MIDI player.
    #[arg(long)]
    pub midi: bool,

    #[clap(verbatim_doc_comment)]
    /// Which MIDI channel to play.
    /// Ignored if --midi option was not used.
    #[arg(short, long, default_value_t = 0)]
    pub channel: u8,
}
