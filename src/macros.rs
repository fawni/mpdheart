// mpdheart, lastfm mpd love client ♡
// Copyright (c) 2024 fawn
//
// SPDX-License-Identifier: Apache-2.0

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        ::std::eprintln!("\x1b[1;31merror\x1b[0;1m: {}", format!($($arg)*));
        std::process::exit(1)
    }
}

#[macro_export]
macro_rules! love {
    ($name:ident, $artist:ident) => {
        if let Err(e) = lastfm::love(&$name, &$artist, true) {
            $crate::err!("failed to love track, http status: {e}");
        } else {
            ::std::println!("\x1b[31m♡ \x1b[0;1m{} \x1b[0;90m{}", $name, $artist)
        }
    };
}

#[macro_export]
macro_rules! unlove {
    ($name:ident, $artist:ident) => {
        if let Err(e) = lastfm::love(&$name, &$artist, false) {
            $crate::err!("failed to unlove track, http status: {e}");
        } else {
            ::std::println!("\x1b[31m❌\x1b[0;1m{} \x1b[0;90m{}", $name, $artist)
        }
    };
}
