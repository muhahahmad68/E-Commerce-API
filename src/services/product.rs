use axum::{Json,
    extract::{State, Path}
    };

use crate::models::product::{Items, ItemRequest, ItemResponse, CategoryRequest, CategoryResponse};
use crate::error::AppError;
pub use crate::state::AppState;

#[axum::debug_handler]
pub async fn create_items(
    State(state): State<AppState>,
    Json(payload): Json<ItemRequest>,
) -> Result<Json<ItemResponse>, AppError> {

    let res = sqlx::query("INSERT INTO items (name, price, category_id) VALUES (?, ?, ?)")
        .bind(&payload.name)
        .bind(&payload.price)
        .bind(&payload.category_id)
        .execute(&state.db)
        .await
        .map_err(|e| format!("Error creating Item: {}", e))?;

    let last_id = res.last_insert_id() as u32;

    let response = ItemResponse {
        id: last_id,
        name: payload.name,
        price: payload.price,
        category_id: payload.category_id,
    };

    Ok(Json(response))

}

#[axum::debug_handler]
pub async fn get_items(
    State(state): State<AppState>,
) -> Result<Json<Vec<ItemResponse>>, AppError> {

    match sqlx::query_as::<_,Items>("SELECT id, name, price, category_id FROM items")
        .fetch_all(&state.db)
        .await
        {
            Ok(items) => {
                let response: Vec<ItemResponse> = items.into_iter().map( |item| ItemResponse {
                    id: item.id,
                    name: item.name,
                    price: item.price,
                    category_id: item.category_id,
                }).collect();

                Ok(Json(response))
            }

            Err (e) => {
                eprintln!("Databse error while fetching item {:?}", e);
                Err(AppError::from(e))
            }
        }

}

pub async fn get_item_by_id(
    State(state): State<AppState>,
    Path(item_id): Path<u32>,
) -> Result<Json<ItemResponse>, AppError> {

    let item = sqlx::query_as::<_, Items>("SELECT id, name, price, category_id FROM items WHERE id = ?")
        .bind(&item_id)
        .fetch_one(&state.db)
        .await
        .map_err(AppError::from)?;

    Ok(Json(ItemResponse{
        id: item.id,
        name: item.name,
        price: item.price,
        category_id: item.category_id,
        }
    ))

}

pub async fn create_category(
    State(state): State<AppState>,
    Json(payload): Json<CategoryRequest>,
) -> Result<Json<CategoryResponse>, AppError> {

    let res = sqlx::query("INSERT INTO categories (name) VALUES (?)")
        .bind(&payload.name)
        .execute(&state.db)
        .await?;

    let last_id = res.last_insert_id() as u32;

    Ok(Json(CategoryResponse {
        id: last_id, // Placeholder, should fetch the last inserted ID
        name: payload.name,
    }))
}

