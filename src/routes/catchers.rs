use rocket::Request;
use rocket_contrib::json::Json;

use crate::errors::ErrorResponse;

#[catch(404)]
pub fn not_found(req: &Request) -> Json<ErrorResponse<()>> {
    let uri = req.uri().to_string();
    // TODO: i18n
    Json(ErrorResponse {
        errmsg: format!(r#"Uri "{}" was not found."#, uri),
        detail: None,
    })
}

#[catch(401)]
pub fn unauthorized(_req: &Request) -> Json<ErrorResponse<()>> {
    // TODO: i18n
    Json(ErrorResponse {
        errmsg: "Unauthorized".into(),
        detail: None,
    })
}
