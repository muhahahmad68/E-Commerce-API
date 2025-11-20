use crate::models::user::UserType;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub sub: u32,
    pub user_type: UserType,
    pub exp: usize,
}
