mod config;
mod controllers;
mod routes;
mod models;

use actix::Actor;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use config::socket_server;
use crate::config::database::establish_connection;
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{cookie::Key, http, middleware, web, App, HttpServer};
use controllers::home;
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use dotenv::dotenv;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = std::env::var("PORT")
        .expect("Port is not defined").parse::<u16>().unwrap();
    let server = socket_server::ChatServer::default().start();
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "DELETE","PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .service(fs::Files::new("/public", "public").show_files_listing())
            .wrap(cors)
            .wrap(middleware::Compress::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build(),
            )
            .app_data(web::Data::new(establish_connection().clone()))
            .app_data(web::Data::new(server.clone()))
            .service(home::index)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
