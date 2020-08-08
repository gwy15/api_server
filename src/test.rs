pub mod prelude {
    pub use crate::errors::ErrMsg;
    pub use crate::schema;
    pub use crate::PgConn;
    use diesel::prelude::*;
    pub use rocket::http::{ContentType, Header, Status};
    pub use rocket::local::{Client, LocalRequest, LocalResponse};
    pub use serde_json::from_str;

    pub fn test_rocket() -> rocket::Rocket {
        let rocket = crate::new_rocket().unwrap();
        let conn = PgConn::get_one(&rocket).unwrap();
        // clean database
        diesel::delete(schema::users::table)
            .execute(&*conn)
            .unwrap();
        // migration
        crate::run_migration(&*conn);
        //
        rocket
    }
}
