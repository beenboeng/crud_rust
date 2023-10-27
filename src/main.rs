#![allow(dead_code)]
extern crate diesel;

pub mod api;
pub mod models;
pub mod routes;
pub mod error;

use actix_web::{web, App, HttpServer};
use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use futures::{select, FutureExt};
use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    // let api_host = dotenv::var("API_HOST").unwrap();
    let config =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new(std::env::var("DATABASE_URL")?);
    let pool = Pool::builder(config).build()?;

    let mut server_future = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::app::config_services)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .fuse();

    //Http Server Notify
    select! {
        _r = server_future => println!("Server is stopped!"),
    };
    Ok(())
}
