use axum::{Json,
    extract::{State, Request},
    middleware::Next,
    response::Response,
    };
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::{Utc, Duration};
use bcrypt;

use crate::error::AppError;
use crate::models::auth::Claims;
use crate::models::user::{LoginRequest, LoginUserResponse, UserType, User};
pub use crate::state::{AppState};

pub async fn auth_middleware(
    State(state): State<AppState>,  
    next: Next,
    mut req: Request,
    )  -> Result<Response, AppError> 

{

        let auth_header = req.headers().get("Authorization").and_then(|e| e.to_str().ok());

        let token = match auth_header {
            Some(h) if h.starts_with("Bearer ") => h.trim_start_matches("Bearer ").to_string(),
            _ => return Err(AppError::Unauthorized("Invalid Authorization header".to_string()))
        };
        
        let token_data = decode::<Claims> (
            &token, 
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()), 
            &Validation::default()
        ).map_err( |_| AppError::Unauthorized("Invalid or expired token".to_string()))?;

        let user_id = token_data.claims.sub;

        req.extensions_mut().insert(user_id);

        Ok(next.run(req).await)
}

pub async fn require_admin(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // First authenticate the user
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(h) if h.starts_with("Bearer ") => &h[7..],
        _ => return Err(AppError::Unauthorized("Missing or invalid token".into())),
    };

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized("Invalid token".into()))?;

    // let user_id = token_data.claims.sub;

    // Check if user is admin
    if token_data.claims.user_type.as_str() != "admin" {
        return Err(AppError::FORBIDDEN("Admin access required".into()));
    }

    req.extensions_mut().insert(token_data.claims);
    Ok(next.run(req).await)
}

pub async fn require_customer(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // First authenticate the user
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(h) if h.starts_with("Bearer ") => &h[7..],
        _ => return Err(AppError::Unauthorized("Missing or invalid token".into())),
    };

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized("Invalid token".into()))?;

    // let user_id = token_data.claims.sub;

    // Check if user is admin
    if token_data.claims.user_type.as_str() != "customer" {
        return Err(AppError::FORBIDDEN("Customer access required".into()));
    }

    req.extensions_mut().insert(token_data.claims);
    Ok(next.run(req).await)
}


#[axum::debug_handler]
pub async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginUserResponse>, AppError> {

    let user = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(&payload.email)
        .fetch_optional(&state.db)
        .await
        {
            Ok(Some(user)) => Ok(user),
            Ok(None) => {
                Err(AppError::DatabaseError("User not found".into()))
            }
            Err(_) => {
                Err(AppError::DatabaseError("User not found".into()))
            }
        }?;
           

    let is_valid = bcrypt::verify(&payload.password, &user.password)
        .map_err(|e| format!("Password verification failed: {}", e))?;
    
    if !is_valid {
        return Err("Invalid email or password".to_string().into());
    }

    let claims = Claims {
        sub: user.id.clone(),
        user_type: UserType::from_str(&user.user_type),
        exp: (Utc::now() + Duration::hours(2)).timestamp() as usize,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(state.jwt_secret.as_ref()))
        .map_err(|e| format!("Token creation failed: {}", e))?;

    let login_user_response = LoginUserResponse {
        id: user.id,
        name: user.name,
        email: user.email,
        token: token,
    };
    
    Ok(Json(login_user_response))
}
