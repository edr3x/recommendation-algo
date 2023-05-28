use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

mod recommendor;

struct AppState {
    db_pool: sqlx::PgPool,
}

#[derive(Serialize, Debug)]
struct NotFound {
    status: bool,
    message: String,
}

async fn not_found() -> impl Responder {
    let response = NotFound {
        status: false,
        message: "404 Not Found".to_string(),
    };

    HttpResponse::NotFound().json(response)
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().expect("can't find .env file");

    let postgres_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let app_data = web::Data::new(AppState {
        db_pool: sqlx::PgPool::connect(postgres_url.as_str())
            .await
            .expect("Failed to connect to Postgres"),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(recommendor::service::config)
            .default_service(web::route().to(not_found))
    })
    .bind("127.0.0.1:5050")?
    .run()
    .await?;

    Ok(())
}
