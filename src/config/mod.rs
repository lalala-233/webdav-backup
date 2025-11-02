pub mod backup;
pub mod webdav;

use std::path::PathBuf;

use crate::prelude::*;
use reqwest_dav::{Auth, Client, ClientBuilder};
use serde::Deserialize;
use sevenz_rust2::{EncoderConfiguration, encoder_options};
use time::{OffsetDateTime, format_description};

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
        let now = OffsetDateTime::now_local()?;
        let format = format_description::parse("[year][month][day]-[hour][minute][second]")?;
        let time = now.format(&format)?;

        let prefix = self
            .backup
            .prefix
            .clone()
            .unwrap_or_else(|| DEFAULT_PREFIX.to_string());

        Ok(format!("{prefix}-{time}.7z"))
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
