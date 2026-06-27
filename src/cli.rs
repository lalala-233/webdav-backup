use crate::prelude::*;
use clap::Parser;
use reqwest_dav::{Client, DecodeError, Depth, StatusMismatchedError};
use sevenz_rust2::ArchiveWriter;
use std::{
    fs,
    io::Cursor,
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
#[clap(version)]
pub struct Cli {
    /// Path to the config file
    #[clap(short, long, default_value = "config.toml")]
    config: PathBuf,
    #[clap(short = 'n', long)]
    /// Print filename that will be uploaded, don't actually compress and upload file
    dry_run: bool,
    /// Path to the directory to compress
    path: PathBuf,
}

impl Cli {
    /// Performs the backup upload.
    ///
    /// # Errors
    /// This function can fail with:
    /// - `Io` – config or archive file I/O errors;
    /// - `Parse` – invalid TOML syntax in config;
    /// - `Sevenz` – 7‑zip compression/decompression errors;
    /// - `WebDAV` – remote server connection or upload failures;
    /// - `PathConversion` – non‑UTF‑8 path encountered.
    pub async fn run(&self) -> Result<(), Error> {
        let config = fs::read_to_string(&self.config)?;
        let config: Config = toml::from_str(&config)?;

        let mut webdav_subfolder = config.get_webdav_subfolder();
        let webdav_subfolder_str = webdav_subfolder.to_str().ok_or(Error::PathConversion)?;

        let filename = config.get_archive_name();
        if self.dry_run {
            let webdav_file_path = webdav_subfolder.to_str().ok_or(Error::PathConversion)?;
            println!("{filename} will be uploaded to {webdav_file_path}");
            return Ok(());
        }

        let client = config.get_webdav_client()?;
        if not_exists(&client, webdav_subfolder_str).await? {
            client.mkcol(webdav_subfolder_str).await?;
        }

        webdav_subfolder.push(filename);
        let webdav_file_path = webdav_subfolder.to_str().ok_or(Error::PathConversion)?;

        let body = get_file(&config, &self.path)?;
        client.put(webdav_file_path, body).await?;

        Ok(())
    }
}

async fn not_exists(client: &Client, path: &str) -> Result<bool, reqwest_dav::Error> {
    let response = client.list(path, Depth::Number(1)).await;
    match response {
        Err(reqwest_dav::Error::Decode(DecodeError::StatusMismatched(StatusMismatchedError {
            response_code: 404,
            ..
        }))) => Ok(true),
        Ok(_) => Ok(false),
        Err(e) => Err(e),
    }
}

fn get_file(config: &Config, path: &Path) -> Result<Vec<u8>, Error> {
    let mut buffer = Cursor::new(Vec::new());
    let mut writer = ArchiveWriter::new(&mut buffer)?;

    writer.set_content_methods(config.get_compress_configuration());
    writer.push_source_path(path, |_| true)?;
    writer.finish()?;

    Ok(buffer.into_inner())
}
