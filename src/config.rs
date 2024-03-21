// mpdheart, lastfm mpd love client ♡
// Copyright (c) 2024 fawn
//
// SPDX-License-Identifier: Apache-2.0

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::consts::{MPD_DEFAULT_HOST, MPD_DEFAULT_PORT};
use crate::Error;
use crate::{err, lastfm};

pub static CONFIG: Lazy<Config> = Lazy::new(Config::load);

// plug a lil weird but he chill
pub static LASTFM_API_SESSION_KEY: Lazy<String> = Lazy::new(|| {
    if let Some(key) = CONFIG.lastfm.session_key.clone() {
        key
    } else {
        let key = lastfm::get_session_key().unwrap_or_else(|e| {
            err!("failed to authenticate on last.fm: {e}");
        });

        let mut cfg = CONFIG.clone();
        cfg.lastfm.session_key = Some(key.clone());
        cfg.save().ok();

        key
    }
});

static CONFIG_DIR: Lazy<String> = Lazy::new(|| {
    if let Some(config_dir) = dirs::config_dir() {
        format!("{}/mpdheart", config_dir.to_string_lossy())
    } else {
        err!("could not locate config dir");
    }
});
static CONFIG_PATH: Lazy<String> = Lazy::new(|| format!("{}/config.toml", *CONFIG_DIR));

#[derive(Deserialize, Serialize, Clone)]
pub struct MpdConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
}

impl Default for MpdConfig {
    fn default() -> Self {
        Self {
            host: Some(MPD_DEFAULT_HOST.to_owned()),
            port: Some(MPD_DEFAULT_PORT),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct LastfmConfig {
    username: String,
    password: String,
    session_key: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Config {
    mpd: Option<MpdConfig>,
    lastfm: LastfmConfig,
}

impl Config {
    pub fn load() -> Self {
        let file = match std::fs::read_to_string(config_path()) {
            Ok(file) => file,
            Err(e) => {
                err!("config file not found: {e}");
            }
        };

        match toml::from_str::<Self>(&file) {
            Ok(config) => config,
            Err(e) => {
                err!("could not deserialize config to toml: {e}");
            }
        }
    }

    fn save(&self) -> Result<(), Error> {
        std::fs::create_dir_all(CONFIG_DIR.clone())?;
        std::fs::write(CONFIG_PATH.clone(), toml::to_string_pretty(self)?)?;

        Ok(())
    }

    pub fn mpd_host(&self) -> String {
        self.mpd
            .clone()
            .and_then(|c| c.host)
            .unwrap_or(MPD_DEFAULT_HOST.to_owned())
    }

    pub fn mpd_port(&self) -> u16 {
        self.mpd
            .clone()
            .and_then(|c| c.port)
            .unwrap_or(MPD_DEFAULT_PORT.to_owned())
    }

    pub fn lastfm_username(&self) -> String {
        if self.lastfm.username.is_empty() {
            err!("empty lastfm username in config");
        }

        self.lastfm.username.clone()
    }

    pub fn lastfm_password(&self) -> String {
        if self.lastfm.password.is_empty() {
            err!("empty lastfm password in config");
        }

        self.lastfm.password.clone()
    }
}

fn config_path() -> String {
    if std::path::Path::new(&*CONFIG_PATH).exists() {
        CONFIG_PATH.clone()
    } else {
        Config::default().save().ok();
        println!("to use mpdheart, edit the config at: {}", *CONFIG_PATH);
        std::process::exit(0);
    }
}
