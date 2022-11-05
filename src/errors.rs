use thiserror::Error;

pub type Result<T> = core::result::Result<T, ManagerError>;

#[derive(Error, Debug)]
pub enum ManagerError {
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("File {0} already exists.")]
    FileExistError(String),

    #[error("Toml de Error: {0}")]
    TomlDeError(#[from] toml::de::Error),

    #[error("Toml ser Error: {0}")]
    TomlGeError(#[from] toml::ser::Error),

    #[error("String Error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}
