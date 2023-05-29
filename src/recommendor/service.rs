use super::models::{UserData, UserResponse, VehicleInfo};

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
