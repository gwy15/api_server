use diesel::result::Error as DError;
use jsonwebtoken::errors::Error as JWTError;

// returned error message
#[derive(Serialize)]
pub struct ErrorResponse<D> {
    pub errmsg: String,
    pub detail: Option<D>,
}

#[derive(Error, Debug)]
pub enum Error {
    /// No Authorization header
    #[error("No login token found in request.")]
    NoLoginToken,

    #[error("Bad token: {}", .0)]
    BadLoginToken(#[from] JWTError),

    #[error("Database error: {}", .0)]
    Database(#[from] DError),
}

pub type Result<T> = std::result::Result<T, Error>;
