use axum::{Json,
    extract::{State, Path},
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

pub async fn get_users (
    State(state): State<AppState>,
) -> Result<Json<Vec<CreateUserResponse>>, AppError> {

    match sqlx::query_as::<_,User>("SELECT id, name, email, user_type FROM users")
        .fetch_all(&state.db)
        .await
        {
            Ok(users) => {
                let response: Vec<CreateUserResponse> = users.into_iter().map( |user| CreateUserResponse {
                    id: user.id,
                    name: user.name,
                    email: user.email,
                    user_type: UserType::from_str(&user.user_type),
                }).collect();

                Ok(Json(response))
            }

            Err (e) => {
                eprintln!("Databse error while fetching users {:?}", e);
                Err(AppError::from(e))
            }
        }

}

pub async fn get_user (
    State(state): State<AppState>,
    Path(user_id): Path<u32>
)   -> Result<Json<CreateUserResponse>, AppError> {

    let user = sqlx::query_as::<_, User>("SELECT id, name, email, user_type FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_one(&state.db)
        .await
        .map_err(AppError::from)?;

    Ok(Json(CreateUserResponse{
        id: user.id,
        name: user.name,
        email: user.email,
        user_type: UserType::from_str(&user.user_type),
        }
    ))

    
}

pub async fn delete_user () {
    
}