use gwy15;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rocket = gwy15::new_rocket()?;

    // launch rocket
    let e = rocket.launch();

    log::error!("Something went wrong: {}", e);
    Err(e)?
}
