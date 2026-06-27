use clap::Parser;
use std::process::exit;
use webdav_backup::cli::Cli;

#[tokio::main]
async fn main() {
    if let Err(error) = Cli::parse().run().await {
        eprintln!("{error}");
        exit(1);
    }
}
