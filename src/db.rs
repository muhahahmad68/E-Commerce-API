use sqlx::mysql::{MySqlPoolOptions};
use sqlx::{MySql, Pool};
// mod config;

use crate::config::Config;

pub async fn init_db () -> Result<Pool<MySql>, sqlx::Error>{
    
    let url = Config::init().database_url;
    
    let pool = MySqlPoolOptions::new().max_connections(5).connect(&url).await;

    Ok(pool?)
}