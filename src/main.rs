use actix_web::{
    get,
    web::{self, Data, Json},
    App, HttpResponse, HttpServer, Responder,
};
use models::schema;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    Pool, Postgres,
};
use std::env;
mod models;
mod service;

#[derive(Debug, Serialize, Deserialize)]
struct HelloMenor {
    h: String,
}

#[get("/api/hello")]
async fn hello_api() -> web::Json<HelloMenor> {
    web::Json(HelloMenor {
        h: "Hello menor".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("database url must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Eror  building conection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello_api)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
