use thiserror::Error;

pub type Result<T> = core::result::Result<T, ManagerError>;

#[derive(Error, Debug)]
pub enum ManagerError {
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("File {0} already exists.")]
    FileExistError(String),
}
