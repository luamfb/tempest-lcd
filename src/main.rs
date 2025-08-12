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

use std::fs;
use clap::Parser;

use gui::Gui;
use args::Args;
use legacy_player::LegacyPlayer;

fn main() {
    let arg_data = Args::parse();
    let filename = &arg_data.filename;
    let file_contents = fs::read_to_string(filename)
        .unwrap_or_else(|e| panic!("failed to read file {}: {}", filename, e));

    let gui = Gui::create(arg_data.horiz_refresh_rate);
    if arg_data.midi {
        panic!("not implemented"); //TODO
    } else {
        let mut player = LegacyPlayer::create(gui, arg_data.cosine);
        let notes = legacy_parser::parse_file_contents(&file_contents);
        player.run(&notes);
    }
}
