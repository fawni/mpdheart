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
    /// Unlove current track instead
    #[arg(short, long)]
    unlove: bool,

    /// Get current track's love status
    #[arg(short, long)]
    status: bool,

    /// Block and append status changes to output
    #[arg(short, long)]
    follow: bool,
}

fn main() {
    let args = Args::parse();
    let (mut name, mut artist) = current();

    if args.status {
        let mut last_status: Option<bool> = None;
        loop {
            let Ok(status) = lastfm::love_status(&name, &artist) else {
                err!("failed to get track's love status");
            };

            if (last_status.is_some() && last_status.unwrap() != status) || last_status.is_none() {
                if status {
                    println!("{}", consts::LOVE_SYMBOL);
                } else {
                    println!("{}", consts::UNLOVE_SYMBOL);
                }
                last_status = Some(status);
            }

            if !args.follow {
                break;
            }

            std::thread::sleep(std::time::Duration::from_millis(1000));
            (name, artist) = current();
        }
    } else if args.unlove {
        unlove!(name, artist);
    } else {
        love!(name, artist);
    };
}

fn current() -> (String, String){
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

    (name, artist)
}
