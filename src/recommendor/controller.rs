use std::collections::HashSet;

use actix_web::{get, web, HttpResponse, Responder};

use crate::recommendor::service::{
    collaborative_filtering_recommendations, content_based_filtering_recommendations,
};
use crate::AppState;

use super::models::{ErrorResponse, SuccessResponse};
use super::service::{user_data, user_history};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: "Route Check".to_string(),
    })
}

#[get("/recom/history/{id}")]
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

    let colab_recoms = collaborative_filtering_recommendations(&all_user_data, &path);

    // let content_recoms = content_based_filtering_recommendations(&all_user_data, &path);

    HttpResponse::Ok().json(SuccessResponse {
        success: true,
        data: colab_recoms,
    })
}

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(index)
        .service(get_users)
        .service(get_recommendations)
        .service(get_collaborative_filtering_recommendations);
}
