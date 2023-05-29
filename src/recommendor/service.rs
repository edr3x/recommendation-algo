use super::models::{UserData, UserResponse, Vehicle, VehicleInfo};

use std::collections::{HashMap, HashSet};

pub async fn user_history(
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

pub async fn user_data() -> Result<Vec<UserData>, Box<dyn std::error::Error>> {
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

fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let dot_product = a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum::<f64>();
    let norm_a = (a.iter().map(|x| x.powi(2)).sum::<f64>()).sqrt();
    let norm_b = (b.iter().map(|x| x.powi(2)).sum::<f64>()).sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0; // To handle division by zero cases
    }

    dot_product / (norm_a * norm_b)
}

fn calculate_user_similarity(target_user: &UserData, other_user: &UserData) -> f64 {
    // Calculate cosine similarity based on user details
    let target_details = [
        target_user.gender.as_deref().unwrap_or(""),
        {
            let this = target_user
                .address
                .as_ref()
                .map(|address| &address.province);
            match this {
                Some(x) => x,
                None => "",
            }
        },
        target_user.email.as_deref().unwrap_or(""),
    ];

    let other_details = [
        other_user.gender.as_deref().unwrap_or(""),
        {
            let this = other_user.address.as_ref().map(|address| &address.province);
            match this {
                Some(x) => x,
                None => "",
            }
        },
        other_user.email.as_deref().unwrap_or(""),
    ];

    let details_similarity = cosine_similarity(
        &target_details
            .iter()
            .map(|s| s.len() as f64)
            .collect::<Vec<f64>>(),
        &other_details
            .iter()
            .map(|s| s.len() as f64)
            .collect::<Vec<f64>>(),
    );

    // Calculate cosine similarity based on booking history
    let target_booking_ids: Vec<&str> = target_user
        .booking
        .iter()
        .map(|booking| booking.vehicle.id.as_str())
        .collect();

    let other_booking_ids: Vec<&str> = other_user
        .booking
        .iter()
        .map(|booking| booking.vehicle.id.as_str())
        .collect();

    let shared_bookings: Vec<&str> = target_booking_ids
        .iter()
        .filter(|&&booking_id| other_booking_ids.contains(&booking_id))
        .copied()
        .collect();

    let bookings_similarity = cosine_similarity(
        &target_booking_ids
            .iter()
            .map(|s| s.len() as f64)
            .collect::<Vec<f64>>(),
        &shared_bookings
            .iter()
            .map(|s| s.len() as f64)
            .collect::<Vec<f64>>(),
    );

    // Combine both similarities with equal weights
    (details_similarity + bookings_similarity) / 2.0
}

pub fn collaborative_filtering_recommendations<'a>(
    users: &'a [UserData],
    target_user_id: &'a str,
) -> Vec<&'a Vehicle> {
    let target_user = users.iter().find(|user| user.id == target_user_id);

    if let Some(user) = target_user {
        let mut user_similarities: HashMap<&str, f64> = HashMap::new();

        for other_user in users {
            if other_user.id == target_user_id {
                continue;
            }

            let similarity = calculate_user_similarity(user, other_user);
            user_similarities.insert(other_user.id.as_str(), similarity);
        }

        let similar_users: Vec<&UserData> = users
            .iter()
            .filter(|user| {
                user.id != target_user_id && user_similarities.contains_key(&user.id.as_str())
            })
            .collect();

        let target_booking_ids: Vec<&str> = user
            .booking
            .iter()
            .map(|booking| booking.vehicle.id.as_str())
            .collect();

        let mut recommended_vehicles: Vec<&Vehicle> = Vec::new();

        for user in similar_users {
            let new_vehicles: Vec<&Vehicle> = user
                .booking
                .iter()
                .filter(|booking| !target_booking_ids.contains(&booking.vehicle.id.as_str()))
                .map(|booking| &booking.vehicle)
                .collect();

            recommended_vehicles.extend(new_vehicles);
        }
        {
            let mut set: HashSet<_> = HashSet::new();
            let mut result = Vec::new();

            for vehicle in recommended_vehicles {
                if set.insert(vehicle.id.clone()) {
                    result.push(vehicle);
                }
            }
            result
        }
    } else {
        Vec::new()
    }
}

pub fn content_based_filtering_recommendations<'a>(
    users: &'a [UserData],
    target_user_id: &'a str,
) -> Vec<&'a Vehicle> {
    let target_user = users.iter().find(|user| user.id == target_user_id);

    if let Some(user) = target_user {
        let booked_vehicle_features: Vec<&str> = user
            .booking
            .iter()
            .flat_map(|booking| booking.vehicle.features.color.split(", "))
            .collect();

        let mut recommended_vehicles: Vec<&Vehicle> = Vec::new();
        for user in users {
            if user.id != target_user_id {
                for booking in &user.booking {
                    if booking
                        .vehicle
                        .features
                        .color
                        .split(", ")
                        .any(|feature| booked_vehicle_features.contains(&feature))
                    {
                        recommended_vehicles.push(&booking.vehicle);
                    }
                }
            }
        }
        {
            let mut set: HashSet<_> = HashSet::new();
            let mut result = Vec::new();

            for vehicle in recommended_vehicles {
                if set.insert(vehicle.id.clone()) {
                    result.push(vehicle);
                }
            }

            result
        }
    } else {
        Vec::new()
    }
}
