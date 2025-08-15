// Copyright (C) 2022 Luana Martins Barbosa
//
// This file is part of tempest-lcd.
// tempest-lcd is free software, released under the
// GNU Public License, version 2 only.
// See COPYING.txt.

mod args;
mod gui;
mod legacy_parser;
mod legacy_player;
mod midi_player;

use std::fs;
use clap::Parser;
use midly::Smf;

use gui::Gui;
use args::Args;
use legacy_player::LegacyPlayer;
use midi_player::MidiPlayer;

fn main() {
    let arg_data = Args::parse();
    let filename = &arg_data.filename;
    let file_contents = fs::read(filename)
        .unwrap_or_else(|e| panic!("failed to read file {}: {}", filename, e));

    let gui = Gui::create(arg_data.horiz_refresh_rate);

    if arg_data.midi {
        let mut player = MidiPlayer::create(gui, arg_data.cosine);
        let smf = Smf::parse(&file_contents)
            .unwrap_or_else(|e| panic!(
                        "failed to parse MIDI file '{}': '{}'",
                        filename,
                        e));
        player.run(smf);
    } else {
        let file_contents_str = String::from_utf8(file_contents)
            .unwrap_or_else(|e| panic!(
                    "failed to convert file '{}' contents to string: '{}'",
                    filename,
                    e));
        let mut player = LegacyPlayer::create(gui, arg_data.cosine);
        let notes = legacy_parser::parse_file_contents(&file_contents_str);
        player.run(&notes);
    }
}
