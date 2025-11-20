use axum::{
    middleware::from_fn_with_state,
    Router,
    routing::{get, post},
};

mod models;
mod state;
mod error;
mod db;
mod services;
mod middleware;
mod config;

use crate::state::AppState;
use crate::services::user::{register_user, get_user, get_users, delete_user};
use crate::services::product::{get_items, get_item_by_id, create_category, create_items};
use crate::services::order::{create_order, get_orders};
use crate::middleware::auth::{login_user, require_customer, require_admin};
use crate::config::Config;

// Error

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let pool = db::init_db().await?;
    let secret = Config::init();

    // Run migrations fresh
    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState { db: pool, jwt_secret: secret.jwt_secret };

    let app = Router::new()
        .route("/", get(server_check))
        .route("/api/register", post(register_user))
        .route("/api/login", post(login_user))
        // customer can only view and create order
        .nest(
            "/api",
            Router::new()
                .route("/items", get(get_items))
                .route("/items/{id}", get(get_item_by_id))
                .route("/orders", post(create_order))
                .route("/orders", get(get_orders))
                .layer(from_fn_with_state(state.clone(), require_customer))
        )
        
        // admin route only
        .nest(
            "/admin/admin",
            Router::new()
                .route("/categories", post(create_category))
                .route("/items", post(create_items))
                .route("/users", get(get_users))
                .route("/users/{id}", get(get_user).delete(delete_user))
                .layer(from_fn_with_state(state.clone(), require_admin))
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}

// middleware

async fn server_check() -> &'static str {
    "Server is running!"
}
