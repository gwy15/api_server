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

    #[error("Form error: {:?}", .0)]
    Form(String),

    #[error("Error: {}", .0)]
    Other(String),
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
            Form(_) => Status::BadRequest,
            Other(_) => Status::BadRequest,
        };

        let error_response = self.to_error_response(false);
        let body = serde_json::to_string(&error_response).unwrap();
        let response = content::Json(body).respond_to(request).unwrap();
        Response::build_from(response).status(status).ok()
    }
}

use rocket::request::FormError;
use rocket::request::{FormDataError, FormParseError};

impl<'f> From<FormError<'f>> for Error {
    fn from(e: FormError) -> Self {
        let s = match e {
            FormDataError::Io(e) => e.to_string(),
            FormDataError::Malformed(_) => "The form was corrupted.".into(),
            FormDataError::Parse(e, _) => match e {
                FormParseError::BadValue(key, _) => format!("Bad form key: \"{}\"", key),
                FormParseError::Unknown(key, _val) => format!("Unknown form key \"{}\"", key),
                FormParseError::Missing(key) => format!("The key \"{}\" was missing.", key),
            },
        };
        Error::Form(s.into())
    }
}
