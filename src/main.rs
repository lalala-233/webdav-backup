use std::process::exit;
use webdav_backup::run;

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("{error}");
        exit(1);
    }
}
