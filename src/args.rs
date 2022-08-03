// Copyright (C) 2022 Luana Martins Barbosa
//
// This file is part of tempest-lcd.
// tempest-lcd is free software, released under the
// GNU Public License, version 2 only.
// See COPYING.txt.

use std::env;

pub struct ArgData {
    pub horiz_refresh_rate: f64,
    pub filename: String,
}

pub fn parse_args() -> ArgData {
    let mut args = env::args().skip(1);
    let horiz_refresh_rate = match args.next() {
        None => {
            usage();
            panic!("missing horizontal refresh rate");
        },
        Some(arg1) => {
            let freq = arg1.parse::<f64>()
                .unwrap_or_else(|e| panic!("1st argument is not a valid f64: {}", e));
            freq * 1000.0 // because freq is in KHz
        },
    };
    let filename = match args.next() {
        None => {
            usage();
            panic!("missing file name");
        },
        Some(arg2) => arg2,
    };
    ArgData {
        horiz_refresh_rate,
        filename,
    }
}

fn usage() {
    println!("usage: <program> <horizontal_refresh_rate_in_KHz> <filename>");
}
