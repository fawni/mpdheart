// mpdheart, lastfm mpd love client ♡
// Copyright (c) 2024 fawn
//
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    MpdError(mpd::error::Error),
    UreqError(Box<ureq::Error>),
    TomlError(toml::ser::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MpdError(e) => write!(f, "{e}"),
            Self::UreqError(e) => write!(f, "{e}"),
            Self::IOError(e) => write!(f, "{e}"),
            Self::TomlError(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<mpd::error::Error> for Error {
    fn from(error: mpd::error::Error) -> Self {
        Self::MpdError(error)
    }
}

impl From<ureq::Error> for Error {
    fn from(error: ureq::Error) -> Self {
        Self::UreqError(Box::new(error))
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(error: toml::ser::Error) -> Self {
        Self::TomlError(error)
    }
}
