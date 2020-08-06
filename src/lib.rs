#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

pub mod models;
pub mod schema;

pub mod errors;
pub mod routes;

#[rocket_contrib::database("pg_db")]
pub struct PgConn(diesel::PgConnection);
