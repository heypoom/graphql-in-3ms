use actix_web::{ get, Responder, web::ServiceConfig };

#[get("/")]
async fn landing() -> impl Responder {
    "Crustacean API is ready!"
}

pub fn landing_route(config: &mut ServiceConfig) {
    config.service(landing);
}
