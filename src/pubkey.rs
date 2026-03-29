use std::env;

use actix_web::{
    HttpResponse, Responder, get,
    web::{self, ServiceConfig},
};
struct PublicKey {
    key: String,
}

fn configure(mut cfg: ServiceConfig) {
    let key = env::var("PUBLIC_KEY").expect("env var PULIC_KEY not set");
    let data = PublicKey { key };
    cfg.service(web::scope("").app_data(web::Data::new(data)));
}
#[get("/.well-known/public-key.pem")]
pub async fn get_public_key(data: web::Data<PublicKey>) -> impl Responder {
    HttpResponse::Ok().body(data.key.clone())
}
