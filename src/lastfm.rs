// mpdheart, lastfm mpd love client ♡
// Copyright (c) 2024 fawn
//
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;

use crate::config::{CONFIG, LASTFM_API_SESSION_KEY};
use crate::consts::{LASTFM_API_KEY, LASTFM_API_ROOT, LASTFM_API_SECRET};
use crate::{err, Error};

fn hash(signature: &str) -> String {
    format!("{:x}", md5::compute(signature))
}

#[derive(Deserialize)]
struct SessionResponse {
    session: Session,
}

#[derive(Deserialize)]
struct Session {
    key: String,
}

pub fn get_session_key() -> Result<String, Error> {
    let method = "auth.getMobileSession";
    let raw_sig = format!(
        "api_key{}method{}password{}username{}{}",
        LASTFM_API_KEY,
        method,
        &CONFIG.lastfm_password(),
        &CONFIG.lastfm_username(),
        LASTFM_API_SECRET
    );

    let response = ureq::request("POST", LASTFM_API_ROOT)
        .set("Content-Length", "0")
        .set("User-Agent", "mpdheart")
        .query("format", "json")
        .query("method", method)
        .query("username", &CONFIG.lastfm_username())
        .query("password", &CONFIG.lastfm_password())
        .query("api_key", LASTFM_API_KEY)
        .query("api_sig", &hash(&raw_sig))
        .call()?
        .into_json::<SessionResponse>()?;

    Ok(response.session.key)
}

pub fn love(track_name: &str, track_artist: &str, love: bool) -> Result<(), Error> {
    let method = if love { "track.love" } else { "track.unlove" };
    let raw_sig = format!(
        "api_key{}artist{}method{}sk{}track{}{}",
        LASTFM_API_KEY,
        track_artist,
        method,
        *LASTFM_API_SESSION_KEY,
        track_name,
        LASTFM_API_SECRET
    );

    let response = ureq::request("POST", LASTFM_API_ROOT)
        .set("Content-Length", "0")
        .set("User-Agent", "mpdheart")
        .query("format", "json")
        .query("method", method)
        .query("track", track_name)
        .query("artist", track_artist)
        .query("api_key", LASTFM_API_KEY)
        .query("api_sig", &hash(&raw_sig))
        .query("sk", &LASTFM_API_SESSION_KEY)
        .call()?;

    if response.status() != 200 {
        err!(
            "failed to love track, http status: {} {}",
            response.status(),
            response.status_text()
        );
    }

    Ok(())
}
