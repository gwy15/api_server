use jsonwebtoken::errors::Error as JWTError;

#[derive(Debug, Error)]
pub enum Error {
    /// No Authorization header
    #[error("No login token found in request.")]
    NoLoginToken,

    #[error("Bad token: {}", .0)]
    BadLoginToken(#[from] JWTError),

    #[error("User not found")]
    UserNotFound,

    #[error("User token expired")]
    TokenExpired,

    #[error("User is banned")]
    Banned,
}
