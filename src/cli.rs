use crate::prelude::*;
use clap::Parser;
use sevenz_rust2::ArchiveWriter;
use std::{
    fs::{self, File},
    path::PathBuf,
};
use time::{OffsetDateTime, format_description};

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
pub fn run() -> Result<(), Error> {
    let args = Cli::parse();

    let config = fs::read_to_string(&args.config)?;
    let config: Config = toml::from_str(&config)?;
    let configuration = config.get_compress_configuration();
    let filename = config.get_archive_name()?;

    // Cursor<Vec<u8>> can be the target of `sevenz::compress_encrypted()`

    let path = std::env::temp_dir().join(filename);
    let writer = File::create(path)?;
    let mut writer = ArchiveWriter::new(writer)?;

    writer.set_content_methods(configuration);
    writer.push_source_path(&args.paths, |_| true)?;
    writer.finish()?;

    Ok(())
}
