use serde::Deserialize;

#[derive(Deserialize)]
pub struct WebDAVConfig {
    pub url: String,
    pub username: String,
    pub password: Option<String>,
}
