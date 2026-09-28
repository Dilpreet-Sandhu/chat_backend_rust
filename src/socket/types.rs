use serde::Deserialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;


#[derive(Deserialize)]
pub struct Params {
    pub user_id : Uuid,
    pub device_id : Uuid,
}

#[derive(FromRow,Clone)]
pub struct Users {
    pub user_id : Uuid
}