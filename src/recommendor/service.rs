use super::models::{UserData, UserResponse, Vehicle, VehicleInfo};

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

pub fn collaborative_filtering_recommendations<'a>(
    users: &'a [UserData],
    target_user_id: &'a str,
) -> Vec<&'a Vehicle> {
    let target_user = users.iter().find(|user| user.id == target_user_id);

    if let Some(user) = target_user {
        let booked_vehicle_ids: Vec<&str> = user
            .booking
            .iter()
            .map(|booking| booking.vehicle.id.as_str())
            .collect();

        let similar_users: Vec<&UserData> = users
            .iter()
            .filter(|&u| u.id != target_user_id)
            .filter(|u| {
                u.booking
                    .iter()
                    .any(|booking| booked_vehicle_ids.contains(&booking.vehicle.id.as_str()))
            })
            .collect();

        let mut recommended_vehicles: Vec<&Vehicle> = Vec::new();
        for user in similar_users {
            recommended_vehicles.extend(
                user.booking
                    .iter()
                    .filter(|booking| !booked_vehicle_ids.contains(&booking.vehicle.id.as_str()))
                    .map(|booking| &booking.vehicle),
            );
        }

        recommended_vehicles
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

        recommended_vehicles
    } else {
        Vec::new()
    }
}
