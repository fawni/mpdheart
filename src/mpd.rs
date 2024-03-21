// mpdheart, lastfm mpd love client ♡
// Copyright (c) 2024 fawn
//
// SPDX-License-Identifier: Apache-2.0

use mpd::{Client, Song};

use crate::config::CONFIG;
use crate::Error;

pub fn current_track() -> Result<Song, Error> {
    let mut client = Client::connect(format!("{}:{}", CONFIG.mpd_host(), CONFIG.mpd_port()))?;
    let current_track = client.currentsong()?;

    if let Some(track) = current_track {
        Ok(track)
    } else {
        println!("nothing is playing :<");
        std::process::exit(0);
    }
}
