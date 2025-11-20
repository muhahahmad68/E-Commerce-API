use serde::{Serialize, Deserialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct Items {
    pub id: u32,
    pub name: String,
    pub price: f32,
    pub category_id: u32,
}

#[derive(Deserialize)]
pub struct ItemRequest {
    pub name: String,
    pub price: f32,
    pub category_id: u32,
}

#[derive(Serialize)]
pub struct ItemResponse {
    pub id: u32,
    pub name: String,
    pub price: f32,
    pub category_id: u32,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: u32,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CategoryRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct CategoryResponse {
    pub id: u32,
    pub name: String,
}
