use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;



#[derive(Deserialize)]
pub struct UserCreateRequest {
    pub username : String,
    pub email : String,
    pub password : String,
    pub avatar : String
}

#[derive(Deserialize)]
pub struct UserLoginRequest {
    pub identifier : String,
    pub password : String
}

#[derive(Serialize)]
pub struct UserLoginResponse {
    pub access_token : String
}

#[derive(Serialize,FromRow)]
pub struct UserResponse {
    pub id : Uuid,
    pub email : String,
    pub username : String,
    pub avatar : String
}