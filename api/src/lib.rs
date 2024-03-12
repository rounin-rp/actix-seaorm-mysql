use actix_cors::Cors;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
//use database::mongodb::{DbName, MongoClient, MongoClientBuilder, Url};
use actix_service::{
    sea_orm::{Database, DatabaseConnection},
    Mutation, Query,
};
use entity::user;
use env_logger::Env;
use migration::{Migrator, MigratorTrait};

//pub mod config;
//pub mod database;
pub mod handlers;
//pub mod helpers;
//pub mod models;
pub mod routes;
//pub mod services;
//pub mod traits;

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[actix_web::main]
async fn start(db_url: String) -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let conn = Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let state = AppState { conn };

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(logger)
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(health_check)
            .service(routes::routes())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

pub fn main(db_url: String) {
    let result = start(db_url);

    if let Some(err) = result.err() {
        print!("Error: {err}");
    }
}
