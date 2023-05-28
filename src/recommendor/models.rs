use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse<T> {
    pub success: bool,
    pub error: T,
}

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Debug)]
pub enum BookingStatus {
    Pending,
    Active,
    Cancelled,
    Rejected,
    Completed,
}

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Booking {
    pub vehicle: Vehicle,
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Vehicle {
    pub id: String,
    pub title: String,
    pub brand: String,
    pub model: String,
    pub features: String,
}

pub struct Features {
    id: i32,
    color: Option<String>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: String,
    pub gender: Option<Gender>,
    pub phone: String,
    pub email: String,
    pub booking: Option<Vec<Booking>>,
    pub address: Option<Address>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Address {
    province: String,
    district: String,
    municipality: String,
    city: String,
    street: Option<String>,
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct VehicleInfo {
    pub id: String,
    pub thumbnail: Option<String>,
    pub title: String,
    pub rate: String,
}
