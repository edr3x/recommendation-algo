use actix_web::{get, web, Responder};

#[get("/")]
async fn index() -> String {
    "Route Check".to_string()
}

#[get("/recom")]
async fn get_recommendations() -> impl Responder {
    // warn: this is a dummy data for now
    let rec = super::models::User {
        id: "1".to_string(),
        name: "John Doe".to_string(),
        email: "john@doe.com".to_string(),
    };

    web::Json(rec)
}

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(index).service(get_recommendations);
}
