#[macro_use]
extern crate log;
extern crate actix_files;
extern crate actix_web;
extern crate fern;
extern crate serde;

use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

/// Configure Rocket to serve on the port requested by Heroku.
fn get_binding_address() -> String {
    if let Ok(port_str) = env::var("PORT") {
        let port: usize = port_str.parse().expect("could not parse PORT");
        return format!("127.0.0.1:{}", port);
    }

    return "127.0.0.1:8888".to_string();
}

#[derive(Deserialize)]
struct Info {
    key: String,
    id: usize,
}

#[derive(Serialize, Deserialize)]
struct Player {
    name: String,
    score: usize,
}

#[derive(Clone)]
struct AppState {
    name: Arc<RwLock<String>>,
    score: Arc<AtomicUsize>,
}


fn show_player_info((info, data): (web::Path<Info>, web::Data<AppState>)) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Player {
        name: data.name.read().unwrap().to_string(),
        score: data.score.load(Ordering::Relaxed)
    }))
}

fn show_player_name((info, data): (web::Path<Info>, web::Data<AppState>)) -> Result<String> {
    Ok(format!("Welcome {} {} {}!", info.key, info.id, data.score.load(Ordering::Relaxed)))
}

fn show_player_score((info, data): (web::Path<Info>, web::Data<AppState>)) -> Result<String> {
    Ok(format!("Welcome {} {} {}!", info.key, info.id, data.score.load(Ordering::Relaxed)))
}

fn set_player_info((info, player, data): (web::Path<Info>, web::Json<Player>, web::Data<AppState>)) -> Result<String> {
    let mut s = data.name.write().unwrap();
    *s = player.name.to_owned();

    data.score.store(player.score, Ordering::Relaxed);
    Ok(format!("Welcome {} {} {} {} {}!", info.key, info.id, player.name, player.score, data.score.load(Ordering::Relaxed)))
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
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
    let path = if dist_path.exists() {
        dist_path
    } else {
        manifest_path
    };

    info!("Running / with directory: {:?}", path);

    let data = AppState {
        name: Arc::new(RwLock::new("".to_string())),
        score: Arc::new(AtomicUsize::new(0)),
    };

    HttpServer::new(move || {
        App::new()
            .route("/again", web::get().to(index2))
            .service(
                web::scope("/api")
                    .data(data.clone())
                    .route("{key}/player/{id}", web::get().to(show_player_info))
                    .route("{key}/player/{id}/name", web::get().to(show_player_name))
                    .route("{key}/player/{id}/score", web::get().to(show_player_score))
                    .route("{key}/player/{id}/update", web::post().to(set_player_info))
            )
            .service(fs::Files::new("/", path).index_file("index.html"))
    })
    .bind(get_binding_address())
    .unwrap()
    .run()
    .unwrap();
}
