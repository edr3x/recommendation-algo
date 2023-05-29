use actix_web::{get, web, HttpResponse, Responder};

use crate::AppState;

use super::models::{ErrorResponse, SuccessResponse, UserData, UserResponse, Vehicle, VehicleInfo};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: "Route Check".to_string(),
    })
}

#[get("/recom/{id}")]
async fn get_recommendations(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let history_data = match user_history(&data.db_pool, path.to_string()).await {
        Ok(data) => data,
        Err(e) => {
            println!("Error getting info: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: e.to_string(),
            });
        }
    };

    if history_data.is_empty() {
        return HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: "No recommendations found".to_string(),
        });
    }

    HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: history_data,
    })
}

#[get("/user")]
async fn get_users() -> impl Responder {
    let all_user_data = match user_data().await {
        Ok(data) => data,
        Err(e) => {
            println!("Error getting info: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: e.to_string(),
            });
        }
    };

    HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: all_user_data,
    })
}

async fn user_history(
    conn: &sqlx::PgPool,
    user_id: String,
) -> Result<Vec<VehicleInfo>, Box<dyn std::error::Error>> {
    let vehicles = sqlx::query_as!(
        VehicleInfo,
        "SELECT DISTINCT ON (b.\"vehicleId\") v.id, v.thumbnail, v.title, v.rate
         FROM \"Booking\" b
         JOIN \"Vehicle\" v ON b.\"vehicleId\" = v.id
         WHERE b.\"bookedById\" = $1
           AND b.\"status\" = 'completed'
           AND b.\"vehicleId\" IN (
             SELECT \"vehicleId\"
             FROM \"Booking\"
             WHERE \"bookedById\" = $1
               AND \"status\" = 'completed'
             GROUP BY \"vehicleId\"
             HAVING COUNT(\"vehicleId\") > 2
         )",
        user_id
    )
    .fetch_all(conn)
    .await?;

    Ok(vehicles)
}

async fn user_data() -> Result<Vec<UserData>, Box<dyn std::error::Error>> {
    let endpoint = std::env::var("USER_ENDPOINT").expect("No endpoint provided");
    let token = std::env::var("USER_TOKEN").expect("No token provided");

    let client = reqwest::Client::new();

    let users = client
        .get(endpoint)
        .header("Authorization", token)
        .send()
        .await?
        .json::<UserResponse>()
        .await?
        .data;

    Ok(users)
}

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(index)
        .service(get_recommendations)
        .service(get_users);
}
