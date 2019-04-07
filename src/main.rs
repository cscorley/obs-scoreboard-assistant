//! A simple Rocket application, based on the example in [Getting Started][].
//!
//! [Getting Started]: https://rocket.rs/v0.4/guide/getting-started/

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate log;
extern crate chrono;
extern crate fern;
extern crate rocket;
extern crate rocket_contrib;

use rocket::{get, routes, Config};
use rocket::response::content;
use rocket_contrib::serve::StaticFiles;
use std::env;
use std::path::Path;

/// Declare a handler.
#[get("/player/<id>/name")]
fn player_name(id: usize) -> content::Json<String> {
    content::Json(format!("\"Player {:?}\"", id))
}

#[get("/player/<id>/score")]
fn player_score(id: usize) -> content::Json<String> {
    content::Json(id.to_string())
}

/// Configure Rocket to serve on the port requested by Heroku.
fn configure() -> Config {
    let mut config = Config::active().expect("could not load configuration");
    if let Ok(port_str) = env::var("PORT") {
        let port = port_str.parse().expect("could not parse PORT");
        config.set_port(port);
    }
    config
}


fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

/// Start our server.
fn main() {
    match setup_logger() {
        Ok(_) => {}
        Err(e) => {
            println!("Got error during logging setup, exiting: {:?}", e);
            return;
        }
    };

    let dist_path = Path::new("./dist");
    let manifest_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/dist"));
    let path = if dist_path.exists() { dist_path } else { manifest_path };

    info!("Running / with directory: {:?}", path);

    rocket::custom(configure())
           .mount("/", StaticFiles::from(path))
           .mount("/api", routes![player_name, player_score])
           .launch();
}
