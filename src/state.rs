use sqlx::{MySql, Pool};

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<MySql>,
    pub jwt_secret: String,
}

