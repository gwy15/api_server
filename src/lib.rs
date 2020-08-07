#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate lazy_static;

pub mod schema;

pub mod errors;
pub use errors::{Error, Result};

mod config;
pub use crate::config::Config;

#[macro_use]
pub mod utils;

pub mod routes;

pub mod account;
pub use account::User;

// type aliases
mod types {
    use rocket::request::{Form as RForm, FormError};
    pub type Form<'r, T> = std::result::Result<RForm<T>, FormError<'r>>;
}
pub use types::Form;

#[rocket_contrib::database("pg_db")]
pub struct PgConn(diesel::PgConnection);

pub fn init_logger() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    log::info!("logging initialized");
}
