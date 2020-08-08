use jsonwebtoken::errors::Error as JWTError;

#[derive(Debug, Error)]
pub enum Error {
    /// No Authorization header
    #[error("No login token found in request.")]
    NoLoginToken,

    #[error("Bad token: {}", .0)]
    BadLoginToken(#[from] JWTError),

    #[error("User ID not found")]
    UserIDNotFound,

    #[error("User token expired")]
    TokenExpired,

    #[error("User is banned")]
    Banned,

    #[error("Username not found or password not matched.")]
    UsernameNotFoundOrPasswordNotMatched,

    #[error("Username \"{}\" occupied", .0)]
    UsernameOccupied(String),
}
