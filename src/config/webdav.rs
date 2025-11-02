use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Clone)]
pub struct WebDAVConfig {
    pub host: String,
    pub subfolder: Option<PathBuf>,
    pub username: String,
    pub password: String,
}
