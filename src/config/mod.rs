pub mod backup;
pub mod webdav;

use crate::prelude::*;
use serde::Deserialize;
use sevenz_rust2::{EncoderConfiguration, encoder_options};
use time::{OffsetDateTime, format_description};

const DEFAULT_COMPRESS_LEVEL: u32 = 6;
const DEFAULT_PREFIX: &str = "backup";

#[derive(Deserialize)]
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
}
