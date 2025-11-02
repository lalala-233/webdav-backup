//! Secure Backup Tool - A secure backup utility with encryption and `WebDAV` upload

pub mod cli;
mod config;
mod error;

pub use cli::run;
mod prelude {
    pub use crate::{
        config::{Config, backup::BackupConfig, webdav::WebDAVConfig},
        error::Error,
    };
}
