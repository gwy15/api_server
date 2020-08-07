/// result to outcome
#[macro_export]
macro_rules! result_to_outcome {
    ($obj:expr => ($status:ident, $err_handle:expr)) => {
        match $obj {
            Ok(o) => o,
            Err(e) => return Outcome::Failure((Status::$status, $err_handle(e).into())),
        }
    };
}

#[macro_export]
macro_rules! option_to_outcome {
    ($obj:expr => ($status:ident, $err_handle:expr)) => {
        match $obj {
            Some(o) => o,
            None => return Outcome::Failure((Status::$status, $err_handle.into())),
        }
    };
}
