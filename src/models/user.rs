use serde::{Serialize, Deserialize};
use sqlx::{Type};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Type, Eq, Copy)]
#[sqlx(type_name="VARCHAR", rename_all="lowercase")]
pub enum UserType {
    Admin,
    Customer
}



impl UserType {
    pub fn as_str(&self) -> &str {
        match self {
            UserType::Admin => "admin",
            UserType::Customer => "customer",
        }
    }

     pub fn from_str(s: &str) -> UserType {
        match s.to_lowercase().as_str() {
            "admin" => UserType::Admin,
            "customer" => UserType::Customer,
             &_ => todo!(),
        }
    }

}

// user model representing a user in the database
#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub user_type: String,
    #[serde(skip_serializing)]
    pub password: String,
}

// create user struct for registration
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub user_type: Option<UserType>,
    pub password: String,
}

// user response struct for responses
#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub user_type: UserType,
}

// login request struct
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// login user response struct
#[derive(Serialize)]
pub struct LoginUserResponse {
    pub id: u32,
    pub name: String,
    pub email:  String,
    pub token: String,
}