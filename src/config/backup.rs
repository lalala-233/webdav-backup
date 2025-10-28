use serde::Deserialize;

#[derive(Deserialize)]
pub struct BackupConfig {
    pub compression_level: Option<u32>,
    pub prefix: Option<String>,
    pub password: String,
}
