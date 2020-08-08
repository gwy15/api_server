pub mod prelude {
    pub use crate::account::User;
    pub use crate::errors::ErrMsg;
    pub use crate::schema;
    pub use crate::PgConn;
    use diesel::prelude::*;
    pub use rocket::http::{ContentType, Header, Status};
    pub use rocket::local::{Client, LocalRequest, LocalResponse};
    pub use serde_json::from_str;

    pub fn setup() -> (Client, PgConn) {
        let rocket = crate::new_rocket().unwrap();
        let conn = PgConn::get_one(&rocket).unwrap();
        // migration
        crate::run_migration(&*conn);
        // clean database
        log::info!("test clean databases");
        diesel::delete(schema::users::table)
            .execute(&*conn)
            .unwrap();
        // create client
        let client = Client::new(rocket).unwrap();
        (client, conn)
    }

    pub fn errmsg_from(mut resp: LocalResponse) -> ErrMsg {
        from_str(&resp.body_string().unwrap()).unwrap()
    }
}
