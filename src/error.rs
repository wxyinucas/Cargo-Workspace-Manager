use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManagerError {
    #[error("I/O error")]
    IoError(#[from] io::Error),
    #[error("Toml de error")]
    TomlDeError(#[from] toml::de::Error),
    #[error("Toml ser error")]
    TomlSerError(#[from] toml::ser::Error),
    #[error("Option with None")]
    NotFoundError,
}
