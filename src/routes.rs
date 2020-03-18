use actix_web::{web, App, HttpResponse, HttpServer};
use crate::controllers::*;

pub fn all(cfg:&mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").configure(api_routes)
    )
    ;
}

fn api_routes(cfg:&mut web::ServiceConfig) {
    cfg.service(
        web::scope("/home").configure(home::default)
    )
    ;
}