use serde::Serialize;
use sqlx::prelude::FromRow;


#[derive(Serialize,FromRow,Clone)]
pub struct User {

    pub id : uuid::Uuid,
    pub email : String,
    pub password : String,
    pub username : String,
    pub avatar : String
}