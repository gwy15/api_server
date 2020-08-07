use super::ErrMsg;
use crate::account::Error as AccountError;
use config::ConfigError;
use diesel::result::Error as DBError;
use rocket::{
    http::Status,
    request::Request,
    response,
    response::{content, Responder, Response},
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

impl<'r> Responder<'r> for Error {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        use Error::*;
        let status = match &self {
            Config(_) => Status::InternalServerError,
            Database(_) => Status::InternalServerError,
            Authorization(_) => Status::Unauthorized,
        };

        let errmsg = self.to_string();
        let body = serde_json::to_string(&ErrMsg::new(errmsg)).unwrap();
        let response = content::Json(body).respond_to(request).unwrap();
        // TODO: add detail under dev
        Response::build_from(response).status(status).ok()
    }
}
