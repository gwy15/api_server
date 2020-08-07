mod msg;
pub use msg::{ErrMsg, ErrorResponse};

use crate::account::Error as AccountError;
use config::ConfigError;
use diesel::result::Error as DError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error loading config file: {}", .0)]
    Config(#[from] ConfigError),

    #[error("Authorization failed.")]
    Authorization(#[from] AccountError),

    #[error("Database error: {}", .0)]
    Database(#[from] DError),
}

pub type Result<T> = std::result::Result<T, Error>;
