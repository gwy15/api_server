use serde_derive::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse<D> {
    pub errmsg: String,
    pub detail: Option<D>,
}
