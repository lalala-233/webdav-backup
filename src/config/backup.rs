use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct BackupConfig {
    pub compression_level: Option<u32>,
    pub prefix: Option<String>,
    pub password: String,
}
