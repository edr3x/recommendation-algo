use serde::{Deserialize, Serialize};

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

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct VehicleInfo {
    pub id: String,
    pub thumbnail: Option<String>,
    pub title: String,
    pub rate: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub success: bool,
    pub data: Vec<UserData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub id: String,
    pub gender: Option<String>,
    pub address: Option<Address>,
    pub email: Option<String>,
    pub booking: Vec<Booking>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub province: String,
    pub district: String,
    pub municipality: String,
    pub city: String,
    pub street: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Booking {
    #[serde(rename = "Vehicle")]
    pub vehicle: Vehicle,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vehicle {
    pub id: String,
    pub title: String,
    pub rate: String,
    pub model: String,
    pub brand: Brand,
    pub category: String,
    pub features: Features,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Brand {
    pub id: String,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features {
    pub id: String,
    pub color: String,
    pub has_airbag: bool,
    #[serde(rename = "hasAC")]
    pub has_ac: bool,
}
