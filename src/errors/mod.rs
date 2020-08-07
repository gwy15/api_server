mod msg;
pub use msg::{ErrMsg, ErrorResponse};

mod error;
pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;
