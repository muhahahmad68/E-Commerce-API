use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Order {
    pub id: i64,
    pub user_id: i64,
    pub product_id: i64,
    pub quantity: i32,
    pub total_price: f64,
    pub status: String,
    pub assigned: bool,
}

#[derive(Deserialize)]
pub struct OrderRequest {
    pub user_id: i64,
    pub product_id: i64,
    pub quantity: i32,
}

#[derive(Serialize)]
pub struct OrderResponse {
    pub user_id: i64,
    pub product_id: i64,
    pub quantity: i32,
    pub total_price: f64,
}