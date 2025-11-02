use crate::prelude::*;
use clap::Parser;
use reqwest_dav::{Client, DecodeError, Depth, StatusMismatchedError};
use sevenz_rust2::ArchiveWriter;
use std::{fs, io::Cursor, path::PathBuf};

#[derive(Parser, Debug)]
#[clap(version)]
struct Cli {
    /// Path to the config file
    #[clap(short, long, default_value = "config.toml")]
    config: PathBuf,

    /// Path to the directory to compress
    paths: PathBuf,
}
/// # Errors
///
/// It will return error if something goes wrong
pub async fn run() -> Result<(), Error> {
    let args = Cli::parse();

    let config = fs::read_to_string(&args.config)?;
    let config: Config = toml::from_str(&config)?;
    let configuration = config.get_compress_configuration();
    let filename = config.get_archive_name()?;
    let mut webdav_subfolder = config.get_webdav_subfolder();

    // Cursor<Vec<u8>> can be the target of `sevenz::compress_encrypted()`

    let mut buffer = Cursor::new(Vec::new());
    let mut writer = ArchiveWriter::new(&mut buffer)?;

    writer.set_content_methods(configuration);
    writer.push_source_path(&args.paths, |_| true)?;
    writer.finish()?;

    let client = config.get_webdav_client()?;

    let webdav_subfolder_str = webdav_subfolder.to_str().ok_or(Error::PathConversion)?;
    if not_exists(&client, webdav_subfolder_str).await? {
        client.mkcol(webdav_subfolder_str).await?;
    }

    webdav_subfolder.push(filename);
    client
        .put(
            webdav_subfolder.to_str().ok_or(Error::PathConversion)?,
            buffer.into_inner(),
        )
        .await?;

    Ok(())
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
