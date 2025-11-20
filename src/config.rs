#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {

    pub fn init() -> Config {

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("SECRET_KEY").expect("Secret key must be set");

        Config {
            database_url,
            jwt_secret,
        }
    }

}