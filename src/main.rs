use unreact::prelude::*;

fn main() -> Result<(), Error> {
    let mut app = Unreact::new(Config::default(), is_dev())?;

    app
        // Index page
        .index("~", object! { secret: "Hello!" })?
        // 404 page
        .not_found("404", object! {})?
        // Complete app
        .run()?;

    // Only prints if NOT in dev mode
    println!("Compiled for production.");
    Ok(())
}
