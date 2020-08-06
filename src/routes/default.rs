use rocket::Request;
use rocket_contrib::json::Json;

use crate::errors::ErrorResponse;

#[derive(Serialize, Debug)]
pub struct NotFound {
    uri: String,
}

impl Into<ErrorResponse<()>> for NotFound {
    fn into(self) -> ErrorResponse<()> {
        // TODO: i18n
        ErrorResponse {
            errmsg: format!(r#"Uri "{}" was not found."#, self.uri),
            detail: None,
        }
    }
}

#[catch(404)]
pub fn not_found(req: &Request) -> Json<ErrorResponse<()>> {
    let uri = req.uri().to_string();

    Json(NotFound { uri }.into())
}
