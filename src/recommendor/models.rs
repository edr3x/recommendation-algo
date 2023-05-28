use serde::Serialize;

#[derive(Debug)]
pub enum BookingStatus {
    Pending,
    Active,
    Cancelled,
    Rejected,
    Completed,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Booking {
    pub id: String,
    pub vehicle: Vehicle,
    pub start_date: String,
    pub end_date: String,
    pub status: BookingStatus,
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Vehicle {
    pub id: String,
    pub title: String,
    pub brand: String,
    pub model: String,
    pub features: String,
}

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: String,
    pub phone: String,
    pub email: String,
    pub vehicle: Vec<Vehicle>,
    pub booking: Vec<Booking>,
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct VehicleInfo {
    pub id: String,
    pub thumbnail: Option<String>,
    pub title: String,
    pub rate: String,
}
