use gwy15;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rocket = gwy15::new_rocket()?;
    let conn = gwy15::PgConn::get_one(&rocket).unwrap();
    gwy15::run_migration(&*conn);

    // launch rocket
    let e = rocket.launch();

    log::error!("Something went wrong: {}", e);
    Err(e)?
}
