mod recommendor;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().expect("can't find .env file");

    // let postgres_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let pool = sqlx::postgres::PgPool::connect(postgres_url.as_str()).await?;

    actix_web::HttpServer::new(move || {
        actix_web::App::new().configure(recommendor::service::config)
    })
    .bind("127.0.0.1:5050")?
    .run()
    .await?;

    Ok(())
}
