use actix_web::{get, web, HttpResponse, Responder};

use crate::AppState;

use super::models::{ErrorResponse, SuccessResponse, User, VehicleInfo};

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

// FIXME: have to fix this
// async fn user_data(conn: &sqlx::PgPool) -> Result<Vec<User>, Box<dyn std::error::Error>> {
//     let users = sqlx::query_as!(
//         User,
//         "SELECT
//             u.id,
//             u.gender,
//             a.province,
//             a.district,
//             a.municipality,
//             a.city,
//             u.phone,
//             u.email,
//             v.id AS vehicle_id,
//             v.title,
//             v.rate,
//             v.model,
//             b_id.id AS brand_id,
//             b_id.title AS brand_title,
//             v.category,
//             f_id.id AS feature_id,
//             f_id.color,
//             f_id.\"hasAirbag\",
//             f_id.\"hasAC\"
//         FROM
//             \"User\" u
//         JOIN
//             \"Booking\" b ON u.id = b.\"bookedById\"
//         JOIN
//             \"Vehicle\" v ON b.\"vehicleId\" = v.id
//         JOIN
//             \"Address\" a ON u.id = a.\"userId\"
//         JOIN
//             \"Brand\" b_id ON v.\"brandId\" = b_id.id
//         JOIN
//             \"VehicleFeature\" f_id ON v.id = f_id.\"vehicleId\"
//         WHERE
//             u.\"isProfileUpdated\" = true
//             AND u.\"isAddressUpdated\" = true
//             AND u.\"isVerified\" = true
//     "
//     )
//     .fetch_all(conn)
//     .await?;

//     Ok(users)
// }

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(index).service(get_recommendations);
}
