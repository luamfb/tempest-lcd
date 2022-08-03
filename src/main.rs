// Copyright (C) 2022 Luana Martins Barbosa
//
// This file is part of tempest-lcd.
// tempest-lcd is free software, released under the
// GNU Public License, version 2 only.
// See COPYING.txt.

mod args;
mod gui;
mod parser;

use std::fs;
use gui::Gui;

fn main() {
    let arg_data = args::parse_args();
    let filename = &arg_data.filename;
    let file_contents = fs::read_to_string(filename)
        .unwrap_or_else(|e| panic!("failed to read file {}: {}", filename, e));

    let notes = parser::parse_file_contents(&file_contents);
    let mut gui = Gui::create(arg_data.horiz_refresh_rate);
    gui.run(&notes);
}
