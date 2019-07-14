#[macro_use]
extern crate log;
extern crate actix_files;
extern crate actix_web;
extern crate chrono;
extern crate diesel;
extern crate dotenv;
extern crate fern;
extern crate obs_scoreboard_assistant;
extern crate serde;

use self::models::*;
use actix_files as fs;
use actix_web::{error, web, App, HttpResponse, HttpServer, Responder, Result};
use diesel::prelude::*;
use obs_scoreboard_assistant::*;
use serde::Deserialize;
use std::env;
use std::path::Path;
use uuid::Uuid;

use self::schema::keys::dsl::*;
use self::schema::players::dsl::*;

/// Configure Rocket to serve on the port requested by Heroku.
fn get_binding_address() -> String {
    if let Ok(port_str) = env::var("PORT") {
        let port: usize = port_str.parse().expect("could not parse PORT");
        return format!("0.0.0.0:{}", port);
    }

    return "0.0.0.0:8888".to_string();
}

#[derive(Deserialize)]
struct Info {
    key: Uuid,
    id: i32,
}

#[derive(Deserialize)]
struct PlayerUpdate {
    name: String,
    score: i16,
}

fn show_player_info((info, pool): (web::Path<Info>, web::Data<Pool>)) -> Result<HttpResponse> {
    let conn = &pool.get().unwrap();
    let key_result = keys
        .filter(key.eq(info.key))
        .load::<Key>(conn)
        .expect("Error loading keys");

    if key_result.len() == 0 {
        return Err(error::ErrorBadRequest("Bad key"));
    }

    let results = players
        .find((info.id, key_result[0].id))
        .load::<Player>(conn)
        .expect("Error loading players");

    if results.len() == 0 {
        return Err(error::ErrorBadRequest("Bad player"));
    }

    Ok(HttpResponse::Ok().json(&results[0]))
}

fn show_player_name((info, pool): (web::Path<Info>, web::Data<Pool>)) -> Result<HttpResponse> {
    let conn = &pool.get().unwrap();
    let key_result = keys
        .filter(key.eq(info.key))
        .load::<Key>(conn)
        .expect("Error loading keys");

    if key_result.len() == 0 {
        return Err(error::ErrorBadRequest("Bad key"));
    }

    let results = players
        .find((info.id, key_result[0].id))
        .load::<Player>(conn)
        .expect("Error loading players");

    if results.len() == 0 {
        return Err(error::ErrorBadRequest("Bad player"));
    }

    Ok(HttpResponse::Ok().json(&results[0].name))
}

fn show_player_score((info, pool): (web::Path<Info>, web::Data<Pool>)) -> Result<HttpResponse> {
    let conn = &pool.get().unwrap();
    let key_result = keys
        .filter(key.eq(info.key))
        .load::<Key>(conn)
        .expect("Error loading keys");

    if key_result.len() == 0 {
        return Err(error::ErrorBadRequest("Bad key"));
    }

    let results = players
        .find((info.id, key_result[0].id))
        .load::<Player>(conn)
        .expect("Error loading players");

    if results.len() == 0 {
        return Err(error::ErrorBadRequest("Bad player"));
    }

    Ok(HttpResponse::Ok().json(results[0].score))
}

fn set_player_info(
    (info, player, pool): (web::Path<Info>, web::Json<PlayerUpdate>, web::Data<Pool>),
) -> Result<HttpResponse> {
    let conn = &pool.get().unwrap();
    // Check for key
    // temp: if no key we can just insert one, who cares for now
    let key_result = keys
        .filter(key.eq(info.key))
        .load::<Key>(conn)
        .expect("Error loading players");

    let new_key_id = if key_result.len() == 0 {
        diesel::insert_into(keys)
            .values(key.eq(info.key))
            .returning(self::schema::keys::dsl::id)
            .get_result(conn)
            .expect("Could not insert new key")
    } else {
        key_result[0].id
    };

    let player_results = players
        .find((info.id, key_id))
        .load::<Player>(conn)
        .expect("Error loading players");

    // Upsert player
    if player_results.len() == 0 {
        diesel::insert_into(players)
            .values((
                self::schema::players::dsl::id.eq(info.id),
                name.eq(&player.name),
                key_id.eq(new_key_id),
                score.eq(player.score),
            ))
            .execute(conn)
            .expect("Could not insert new player");
    } else {
        diesel::update(players.find((info.id, key_id)))
            .set((
                name.eq(&player.name),
                score.eq(player.score),
                self::schema::players::dsl::updated_on.eq(chrono::Utc::now()),
            ))
            .execute(conn)
            .expect("could not update player");
    }

    let player_results = players
        .find((info.id, key_id))
        .load::<Player>(conn)
        .expect("Error loading players");

    Ok(HttpResponse::Ok().json(&player_results[0]))
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

    for var in env::vars() {
        info!("Environment variable {:?}: {:?}", var.0, var.1);
    }

    let dist_path = Path::new("./dist");
    let manifest_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/dist"));
    let path = if dist_path.exists() {
        dist_path
    } else {
        manifest_path
    };

    info!("Running / with directory: {:?}", path);

    let pool = establish_connection();

    HttpServer::new(move || {
        App::new()
            .route("/again", web::get().to(index2))
            .service(
                web::scope("/api")
                    .data(pool.clone())
                    .route("{key}/player/{id}", web::get().to(show_player_info))
                    .route("{key}/player/{id}/name", web::get().to(show_player_name))
                    .route("{key}/player/{id}/score", web::get().to(show_player_score))
                    .route("{key}/player/{id}/update", web::post().to(set_player_info)),
            )
            .service(fs::Files::new("/", path).index_file("index.html"))
    })
    .bind(get_binding_address())
    .unwrap()
    .run()
    .unwrap();
}
