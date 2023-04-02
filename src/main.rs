use unreact::prelude::*;

// Where the site is hosted
const URL: &str = "https://darccyy.github.io/unreact-template";

fn main() -> Result<(), Error> {
    let mut app = Unreact::new(Config::default(), is_dev(), URL)?;

    app
        // Index page
        .index("index", object! { secret: "Hello!" })
        // 404 page
        .not_found("404", object! {});

    // Custom page
    app.route(
        "hello/there",
        "hello",
        object! { list: vec![1, 2, 3, 4, 5] },
    );

    // Complete app
    app.run()?;

    // Only prints if NOT in dev mode
    println!("Compiled for production.");
    Ok(())
}
