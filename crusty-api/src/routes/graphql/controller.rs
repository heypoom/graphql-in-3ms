#![feature(async_closure)]


use std::sync::Arc;

use actix_web::{Error, HttpResponse, web, get, post, Result, web::ServiceConfig, HttpRequest};

use juniper::http::GraphQLRequest;
use juniper::http::graphiql::graphiql_source;

use crate::services::schema::Schema;
use juniper_graphql_ws::ConnectionConfig;
use tokio::time::Duration;
use juniper_actix::subscriptions::subscriptions_handler;

#[get("/graphiql")]
pub async fn graphiql() -> HttpResponse {
    let source = graphiql_source(
        "/graphql",
        Some("/subscriptions"));

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source)
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

#[get("/subscriptions")]
async fn subscriptions(
    schema: web::Data<Arc<Schema>>,
    request: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let config = ConnectionConfig::new(());

    // set the keep alive interval to 15 secs so that it doesn't timeout in playground
    // playground has a hard-coded timeout set to 20 secs
    let config = config.with_keep_alive_interval(Duration::from_secs(15));

    let rootNode = (*schema.into_inner()).clone();

    subscriptions_handler(request, stream, rootNode, config).await

    // Ok(HttpResponse::Ok().json("{}"))
}

pub fn graphql_route(config: &mut ServiceConfig) {
    config
        .service(graphiql)
        .service(graphql)
        .service(subscriptions);
}
