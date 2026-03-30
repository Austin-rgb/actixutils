use actix_web::{
    HttpResponse, Responder, get,
    web::{self, ServiceConfig},
};

use crate::utils::get_public_key;
struct PublicKey {
    key: String,
}

pub fn configure(cfg: &mut ServiceConfig) {
    let key = get_public_key();
    let data = PublicKey { key };
    cfg.service(
        web::scope("")
            .app_data(web::Data::new(data))
            .service(public_key),
    );
}
#[get("/.well-known/public-key.pem")]
async fn public_key(data: web::Data<PublicKey>) -> impl Responder {
    HttpResponse::Ok().body(data.key.clone())
}
