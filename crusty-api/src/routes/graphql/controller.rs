use std::sync::Arc;

use actix_web::{Error, HttpResponse, web, get, post, Result, web::ServiceConfig};

use juniper::http::GraphQLRequest;
use juniper::http::graphiql::graphiql_source;

use crate::services::schema::Schema;

#[get("/graphiql")]
pub async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            graphiql_source("/graphql", None)
        )
}

#[post("/graphql")]
pub async fn graphql(
    data: web::Data<Arc<Schema>>,
    request: web::Json<GraphQLRequest>
) -> Result<HttpResponse, Error> {
    let res = request.execute(&data, &()).await;

    Ok(HttpResponse::Ok()
        .json(res)
    )
}

pub fn graphql_route(config: &mut ServiceConfig) {
    config
        .service(graphiql)
        .service(graphql);
}
