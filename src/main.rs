#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

// make diesel migration module
#[macro_use]
extern crate diesel_migrations;
embed_migrations!();

fn main() {
    dotenv::dotenv().ok();

    // gwy15::init_logger();

    // build rocket
    let rocket = rocket::ignite()
        .attach(gwy15::PgConn::fairing())
        .mount("/hello", routes![gwy15::routes::index::index])
        .register(catchers![gwy15::routes::default::not_found]);

    // Run db migrations
    let db_con =
        gwy15::PgConn::get_one(&rocket).expect("Failed to get a db connection for migration.");
    // embedded_migrations::run(&connection);
    embedded_migrations::run_with_output(&*db_con, &mut std::io::stdout())
        .expect("migration failed.");
    log::info!("Database migration finished.");

    // launch rocket
    rocket.launch();
}
