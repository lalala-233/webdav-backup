pub mod backup;
pub mod webdav;

use std::path::PathBuf;

use crate::prelude::*;
use reqwest_dav::{Auth, Client, ClientBuilder};
use serde::Deserialize;
use sevenz_rust2::{EncoderConfiguration, encoder_options};
use time::{OffsetDateTime, macros::format_description};

const DEFAULT_COMPRESS_LEVEL: u32 = 6;
const DEFAULT_PREFIX: &str = "backup";
const DEFAULT_SUBFOLDER: &str = "/dav/backup/";

#[derive(Deserialize, Clone)]
pub struct Config {
    backup: BackupConfig,
    webdav: WebDAVConfig,
}

impl Config {
    pub fn get_compress_configuration(&self) -> Vec<EncoderConfiguration> {
        vec![
            encoder_options::AesEncoderOptions::new(self.backup.password.as_str().into()).into(),
            encoder_options::Lzma2Options::from_level(
                self.backup
                    .compression_level
                    .unwrap_or(DEFAULT_COMPRESS_LEVEL),
            )
            .into(),
        ]
    }
    pub fn get_archive_name(&self) -> Result<String, time::Error> {
        let now = OffsetDateTime::now_utc();
        let time = now.format(&format_description!(
            version = 3,
            "[year]-[month]-[day]T[hour]:[minute]:[second]Z"
        ))?;

        Ok(format!(
            "{}-{time}.7z",
            self.backup.prefix.as_deref().unwrap_or(DEFAULT_PREFIX)
        ))
    }
    pub fn get_webdav_client(&self) -> Result<Client, Error> {
        let webdav = self.webdav.clone();
        Ok(ClientBuilder::new()
            .set_host(webdav.host)
            .set_auth(Auth::Basic(webdav.username, webdav.password))
            .build()?)
    }
    pub fn get_webdav_subfolder(&self) -> PathBuf {
        self.webdav
            .subfolder
            .clone()
            .unwrap_or_else(|| PathBuf::from(DEFAULT_SUBFOLDER))
    }
}
