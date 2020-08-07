// returned error message
#[derive(Serialize)]
pub struct ErrorResponse<D> {
    pub errmsg: String,
    pub detail: Option<D>,
}

pub type ErrMsg = ErrorResponse<()>;

impl ErrMsg {
    pub fn new<T>(errmsg: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            errmsg: errmsg.into(),
            detail: None,
        }
    }
}
