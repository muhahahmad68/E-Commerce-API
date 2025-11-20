use axum::{Json,
    extract::{State, Extension}
    };

use crate::models::order::{Order, OrderRequest, OrderResponse};
use crate::models::user::{User};
use crate::error::AppError;
pub use crate::state::AppState;

#[axum::debug_handler]
pub async fn create_order(
    State(state): State<AppState>,
    Json(payload): Json<OrderRequest>,
) -> Result<Json<OrderResponse>, AppError> {

    let price_opt = sqlx::query_scalar::<_, f64> ("SELECT price FROM items WHERE id = ?")
        .bind(payload.product_id)
        .fetch_optional(&state.db)
        .await?;
    
    let price = price_opt.unwrap_or(0.0);

    let total_price = price * payload.quantity as f64;

    sqlx::query("INSERT INTO order (user_id, product_id, quantity, total_price)")
        .bind(&payload.user_id)
        .bind(&payload.product_id)
        .bind(&payload.quantity)
        .bind(&total_price)
        .execute(&state.db)
        .await?;

    let response = OrderResponse {
        user_id: payload.user_id,
        product_id: payload.product_id,
        quantity: payload.quantity,
        total_price: total_price,

    };

    Ok(Json(response))
}

#[axum::debug_handler]
pub async fn get_orders(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
) -> Result<Json<Vec<OrderResponse>>, AppError> {   

    let user_id = current_user.id;

    match sqlx::query_as::<_, Order> ("SELECT * FROM order WHERE user_id = ?")
        .bind(user_id)
        .fetch_all(&state.db)
        .await
        {
            Ok(orders) => {
                let response: Vec<OrderResponse> = orders.into_iter().map( |order| OrderResponse {
                    user_id: order.user_id,
                    product_id: order.product_id,
                    quantity: order.quantity,
                    total_price: order.total_price,
                }).collect();

                Ok(Json(response))
            }

            Err (e) => {
                eprintln!("Error while fetching orders {:?}", e);
                Err(AppError::from(e))
            }
        }
}
