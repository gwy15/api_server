use super::ErrorResponse;
use crate::account::Error as AccountError;
use config::ConfigError;
use diesel::result::Error as DBError;
use rocket::{
    http::Status,
    request::Request,
    response::{content, Responder, Response, Result as ResponseResult},
};
use serde_json;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error loading config file: {}", .0)]
    Config(#[from] ConfigError),

    #[error("Authorization failed: {}", .0)]
    Authorization(#[from] AccountError),

    #[error("Database error: {}", .0)]
    Database(#[from] DBError),
}

impl Error {
    fn to_error_response(&self, detailed: bool) -> ErrorResponse<String> {
        let detail = if detailed { unimplemented!() } else { None };
        // TODO: add detail under dev
        ErrorResponse {
            errmsg: self.to_string(),
            detail,
        }
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, request: &Request) -> ResponseResult<'r> {
        use Error::*;
        let status = match &self {
            Config(_) => Status::InternalServerError,
            Database(_) => Status::InternalServerError,
            Authorization(_) => Status::Unauthorized,
        };

        let error_response = self.to_error_response(false);
        let body = serde_json::to_string(&error_response).unwrap();
        let response = content::Json(body).respond_to(request).unwrap();
        Response::build_from(response).status(status).ok()
    }
}
