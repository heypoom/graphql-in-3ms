#[macro_use]
extern crate lazy_static;
extern crate juniper;

mod models;
mod services;
mod routes;

use actix_cors::Cors;
use actix_web::{http, middleware::Compress, App, HttpServer};

use std::sync::Arc;

use services::schema::create_schema;
use routes::graphql::controller::graphql_route;
use routes::landing::controller::landing_route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Arc::new(create_schema());

    HttpServer::new(move || {
        // Allow fetch requests from any domain.
        let cors = Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
            .max_age(86400);

        App::new()
            .wrap(cors)
            .wrap(Compress::default())
            .data(schema.clone())
            .configure(landing_route)
            .configure(graphql_route)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
