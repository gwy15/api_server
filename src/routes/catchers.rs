use rocket::Request;
use rocket_contrib::json::Json;

use crate::errors::ErrMsg;

#[catch(404)]
pub fn not_found(req: &Request) -> Json<ErrMsg> {
    let uri = req.uri().to_string();
    // TODO: i18n
    Json(ErrMsg::new(format!(r#"Uri "{}" was not found."#, uri)))
}

#[catch(401)]
pub fn unauthorized(_req: &Request) -> Json<ErrMsg> {
    // TODO: i18n
    Json(ErrMsg::new("Unauthorized"))
}
