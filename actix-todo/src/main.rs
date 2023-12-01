mod config;
mod db;
mod errors;
mod handlers;
mod models;

use crate::config::Config;
use crate::handlers::*;
use crate::models::AppState;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use slog::info;
use std::io;
use tokio_postgres::NoTls;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    let log = Config::configure_log();

    info!(
        log,
        "Starting server at http://{}:{}", config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                pool: pool.clone(),
                log: log.clone(),
            })
            .wrap(middleware::Logger::default())
            // below route will be used with curl as curl -X GET http://localhost:8080/
            .route("/", web::get().to(status))
            // below route will be used with curl as curl -X GET http://localhost:8080/todos
            .route("/todos{_:/?}", web::get().to(todos))
            // below route will be used with curl as curl -X POST http://localhost:8080/todos -d '{"title":"New Todo"}' -H 'Content-Type: application/json'
            .route("/todos{_:/?}", web::post().to(create_todo))
            // below route will be used with curl as curl -X GET http://localhost:8080/todos/1
            .route("/todos/{list_id}{_:/?}", web::get().to(get_todo))
            // below route will be used with curl as curl -X GET http://localhost:8080/todos/1/items
            .route("/todos/{list_id}/items{_:/?}", web::get().to(items))
            // below route will be used with curl as curl -X POST http://localhost:8080/todos/1/items -d '{"title":"New Todo Item"}' -H 'Content-Type: application/json'
            .route("/todos/{list_id}/items{_:/?}", web::post().to(create_item))
            // below route will be used with curl as curl -X GET http://localhost:8080/todos/1/items/1
            .route(
                "/todos/{list_id}/items/{item_id}{_:/?}",
                web::get().to(get_item),
            )
            // below route will be used with curl as curl -X PUT http://localhost:8080/todos/1/items/1`
            // in put body we need to pass {"checked":true} or {"checked":false}
            .route(
                "/todos/{list_id}/items/{item_id}{_:/?}",
                web::put().to(check_todo),
            )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}

#[cfg(test)]
#[cfg(feature = "integration")]
mod integration_tests;
