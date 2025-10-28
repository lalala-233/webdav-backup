use webdav_backup::run;
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
    }
}
