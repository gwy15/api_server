#[derive(Serialize)]
pub struct ErrorResponse<D> {
    pub errmsg: String,
    pub detail: Option<D>,
}
