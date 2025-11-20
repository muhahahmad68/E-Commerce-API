use axum::{Json,
    extract::State,
    };

use crate::models::user::{User, CreateUserRequest, CreateUserResponse, UserType};
use crate::error::AppError;
pub use crate::state::AppState;

#[axum::debug_handler]
pub async fn register_user(
    State(state): State<AppState>, 
    Json(payload): Json<CreateUserRequest>
) -> Result<Json<CreateUserResponse>, AppError> {

    let user_type = payload.user_type.unwrap_or(UserType::Customer);
    
    let existing_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(&payload.email)
        .fetch_optional(&state.db)
        .await?;
    
    if existing_user.is_some() {
        return Err("Email already registered".to_string().into());
    }

    let hashed_password = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)
        .map_err(|e| format!("Password hashing failed: {}", e))?;

    let res = sqlx::query("INSERT INTO users (name, email, password, user_type) VALUES (?, ?, ?, ?)")
        .bind(&payload.name)
        .bind(&payload.email)
        .bind(&hashed_password)
        .bind(user_type.as_str())
        .execute(&state.db)
        .await?;
    
    let user_id = res.last_insert_id() as u32;

    let user_response = CreateUserResponse {
        id: user_id,
        name: payload.name,
        email: payload.email,
        user_type,
    };

    Ok(Json(user_response))
}

pub async fn get_users () {

}

pub async fn get_user () {
    
}

pub async fn delete_user () {
    
}