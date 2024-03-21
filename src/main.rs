// mpdheart, lastfm mpd love client ♡
// Copyright (c) 2024 fawn
//
// SPDX-License-Identifier: Apache-2.0

use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};

mod config;
mod consts;
mod lastfm;
mod macros;
mod mpd;

type Error = Box<dyn std::error::Error>;

const fn clap_style() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Yellow.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default())
}

/// mpdheart, spread love around your library ♡
#[derive(Parser)]
#[clap(version, author, styles = clap_style())]
struct Args {
    /// Unlove instead
    #[arg(short, long)]
    unlove: bool,
}

fn main() {
    let args = Args::parse();
    let current_track = match mpd::current_track() {
        Ok(track) => track,
        Err(e) => {
            err!("{e}");
        }
    };

    let Some(name) = current_track.title else {
        err!("could not determine track name");
    };
    let Some(artist) = current_track.artist else {
        err!("could not determine artist name");
    };

    if args.unlove {
        unlove!(name, artist);
    } else {
        love!(name, artist);
    };
}
