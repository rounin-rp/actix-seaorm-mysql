use actix_web::web;

pub mod user_routes;

use super::AppState;

pub fn routes() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("api").service(user_routes::routes())
}
