// mpdheart, lastfm mpd love client ♡
// Copyright (c) 2024 fawn
//
// SPDX-License-Identifier: Apache-2.0

pub const LOVE_SYMBOL: char       = '♥';
pub const UNLOVE_SYMBOL: char     = '♡';

pub const MPD_DEFAULT_HOST: &str  = "0.0.0.0";
pub const MPD_DEFAULT_PORT: u16   = 6600;

pub const LASTFM_API_ROOT: &str   = "http://ws.audioscrobbler.com/2.0";
// ideally this would be hidden but i can't think of a way to do that.
// scrubbler also exposes these so it should be fine:
// https://github.com/SHOEGAZEssb/Last.fm-Scrubbler-WPF/blob/a7fb3105ad711891b1e731f89fee8420cb1b8aed/Last.fm-Scrubbler-WPF/AppBootstrapper.cs#L18-L26
pub const LASTFM_API_KEY: &str    = "c0566c477189ccd19f55b3a0d5213629";
pub const LASTFM_API_SECRET: &str = "dbc72a5d3020582afcb878eaecccfb7f";
