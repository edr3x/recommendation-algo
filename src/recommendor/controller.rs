use actix_web::{get, web, HttpResponse, Responder};

use super::models::{ErrorResponse, SuccessResponse, VehicleResponse};
use super::service::{collaborative_filtering_recommendations, user_data};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: "Route Check".to_string(),
    })
}

// #[get("/recom/history/{id}")]
// async fn get_recommendations(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
//     let history_data = match user_history(&data.db_pool, path.to_string()).await {
//         Ok(data) => data,
//         Err(e) => {
//             println!("Error getting info: {}", e);
//             return HttpResponse::InternalServerError().json(ErrorResponse {
//                 success: false,
//                 error: e.to_string(),
//             });
//         }
//     };

//     if history_data.is_empty() {
//         return HttpResponse::NotFound().json(ErrorResponse {
//             success: false,
//             error: "No recommendations found".to_string(),
//         });
//     }

//     HttpResponse::Ok().json(SuccessResponse {
//         success: true,
//         data: history_data,
//     })
// }

#[get("/recom/{id}")]
async fn get_collaborative_filtering_recommendations(path: web::Path<String>) -> impl Responder {
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

    let (colab_recoms, _) = collaborative_filtering_recommendations(&all_user_data, &path);

    // let content_recoms = content_based_filtering_recommendations(&all_user_data, &path);

    let vehicle_responses: Vec<VehicleResponse> = colab_recoms
        .iter()
        .map(|vehicle| VehicleResponse {
            id: vehicle.id.clone(),
            title: vehicle.title.clone(),
            rate: vehicle.rate.clone(),
            thumbnail: vehicle.thumbnail.clone(),
        })
        .collect();

    HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: vehicle_responses,
    })
}

#[derive(serde::Serialize)]
struct UserStruct {
    id: String,
    similarity: f64,
}

#[get("/similaruser/{id}")]
async fn similar_user(path: web::Path<String>) -> impl Responder {
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

    let (_, similar_users) = collaborative_filtering_recommendations(&all_user_data, &path);

    let response_similar_user: Vec<UserStruct> = similar_users
        .clone()
        .into_iter()
        .map(|(id, similarity)| UserStruct {
            id: id.to_string(),
            similarity,
        })
        .collect();

    HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: response_similar_user,
    })
}

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(index)
        .service(similar_user)
        .service(get_collaborative_filtering_recommendations);
}
