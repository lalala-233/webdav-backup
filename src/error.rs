use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse Error: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("7z Error: {0}")]
    Sevenz(#[from] sevenz_rust2::Error),
    #[error("Time Error: {0}")]
    Time(#[from] time::Error),
}
