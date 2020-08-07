#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

// make diesel migration module
#[macro_use]
extern crate diesel_migrations;
embed_migrations!();

use gwy15::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let config = Config::new()?;

    // gwy15::init_logger();

    let routes = routes![
        gwy15::routes::index::index,
        gwy15::routes::index::user,
        gwy15::account::routes::login,
    ];
    let catchers = catchers![
        gwy15::routes::catchers::not_found,
        gwy15::routes::catchers::unauthorized,
    ];

    // build rocket
    let rocket = rocket::ignite()
        .manage(config)
        .attach(gwy15::PgConn::fairing())
        .mount("/hello", routes)
        .register(catchers);

    // Run db migrations
    let db_con =
        gwy15::PgConn::get_one(&rocket).expect("Failed to get a db connection for migration.");
    // embedded_migrations::run(&connection);
    embedded_migrations::run_with_output(&*db_con, &mut std::io::stdout())
        .expect("migration failed.");
    log::info!("Database migration finished.");

    // launch rocket
    let e = rocket.launch();

    log::error!("Something went wrong: {}", e);
    Err(e)?
}
