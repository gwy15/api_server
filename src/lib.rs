#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

pub mod schema;
pub mod models;

pub mod errors;

pub mod routes;
