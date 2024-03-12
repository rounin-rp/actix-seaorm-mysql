use super::AppState;
use crate::handlers::error_handler::Errors;
use actix_service::{Mutation, Query};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, ResponseError};
use entity::user;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Params {
    page: Option<u64>,
    users_per_page: Option<u64>,
}
#[post("/")]
async fn create(data: web::Data<AppState>, post_form: web::Json<user::Model>) -> impl Responder {
    let conn = &data.conn;

    let form = post_form.into_inner();
    let response = Mutation::create_user(conn, form)
        .await
        .map_err(|err| Errors::InternalError(err.to_string()));
    match response {
        Ok(user) => HttpResponse::Created().json("Created"),
        Err(error) => error.error_response(),
    }
}
#[get("/all")]
async fn get_all_users(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let conn = &data.conn;

    let params = web::Query::<Params>::from_query(req.query_string()).unwrap();

    let page = params.page.unwrap_or(1);
    let users_per_page = params.users_per_page.unwrap_or(10);

    let response = Query::find_users_in_page(conn, page, users_per_page)
        .await
        .map_err(|err| Errors::InternalError(err.to_string()));

    match response {
        Ok((users, total_pages)) => HttpResponse::Ok().json(users),
        Err(error) => error.error_response(),
    }
}
pub fn routes() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("users").service(create).service(get_all_users)
}
