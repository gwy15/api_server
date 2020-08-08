use crate::errors::ErrMsg;
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

    #[error("Password too weak")]
    PasswordTooWeak,
}

use rocket::{
    http::Status,
    request::Request,
    response::{content, Responder, Response, Result as ResponseResult},
};
use serde_json;
impl<'r> Responder<'r> for Error {
    fn respond_to(self, request: &Request) -> ResponseResult<'r> {
        use Error::*;
        let status = match &self {
            UsernameOccupied(_) | PasswordTooWeak => Status::BadRequest,
            Banned => Status::Forbidden,
            _ => Status::Unauthorized,
        };

        let error_response = ErrMsg::new(self.to_string());
        let body = serde_json::to_string(&error_response).unwrap();
        let response = content::Json(body).respond_to(request).unwrap();
        Response::build_from(response).status(status).ok()
    }
}
