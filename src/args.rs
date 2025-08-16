// Copyright (C) 2022 Luana Martins Barbosa
//
// This file is part of tempest-lcd.
// tempest-lcd is free software, released under the
// GNU Public License, version 2 only.
// See COPYING.txt.

#[derive(clap::Parser)]
pub struct Args {
    pub horiz_refresh_rate: f64,
    pub filename: String,
    #[arg(long)]
    pub cosine: bool,
    #[arg(long)]
    pub midi: bool,
    #[arg(short, long, default_value_t = 0)]
    pub channel: u8,
}
