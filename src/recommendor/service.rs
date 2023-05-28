use actix_web::{get, web, HttpResponse, Responder};

use crate::AppState;

use super::models::VehicleInfo;

#[get("/")]
async fn index() -> String {
    "Route Check".to_string()
}

#[get("/recom/{id}")]
async fn get_recommendations(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let history_data = match user_history(&data.db_pool, path.to_string()).await {
        Ok(data) => data,
        Err(e) => {
            println!("Error getting info: {}", e);
            Vec::new()
        }
    };

    HttpResponse::Ok().json(history_data)
}

async fn user_history(
    conn: &sqlx::PgPool,
    userId: String,
) -> Result<Vec<VehicleInfo>, Box<dyn std::error::Error>> {
    // let query = r#"SELECT DISTINCT ON (b.\"vehicleId\") v.id, v.thumbnail, v.title, v.rate
    //                 FROM \"Booking\" b
    //                 JOIN \"Vehicle\" v ON b.\"vehicleId\" = v.id
    //                 WHERE b.\"bookedById\" = $1
    //                   AND b.\"status\" = 'completed'
    //                   AND b.\"vehicleId\" IN (
    //                     SELECT \"vehicleId\"
    //                     FROM \"Booking\"
    //                     WHERE \"bookedById\" = $1
    //                       AND \"status\" = 'completed'
    //                     GROUP BY \"vehicleId\"
    //                     HAVING COUNT(\"vehicleId\") > 2
    //                 )"#;

    // let test_query = "SELECT id, thumbnail, title, rate FROM \"Vehicle\"";

    // let history = sqlx::query_as!(VehicleInfo, test_query)
    //     .fetch_all(conn)
    //     .await?;

    let vehicle_test = sqlx::query_as!(
        VehicleInfo,
        "SELECT id, thumbnail, title, rate FROM \"Vehicle\""
    )
    .fetch_all(conn)
    .await?;

    Ok(vehicle_test)
}

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(index).service(get_recommendations);
}
