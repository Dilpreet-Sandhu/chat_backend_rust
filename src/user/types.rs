use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;



#[derive(Deserialize)]
pub struct CreateNewDeviceType {
    pub public_key : String,
    pub user_id : String
}


pub struct CreateNewDeviceServiceType { 
    pub public_key : Vec<u8>,
    pub user_id : Uuid
}


#[derive(Deserialize)]
pub struct GetPublicKeyType {
    pub user_id : String
}


pub struct GetPublicKeyServiceType {
    pub user_id : Uuid
}


#[derive(FromRow)]
pub struct UserDevicePublicKeys {
    pub public_key : Vec<u8>,
    pub device_id : Uuid
}

#[derive(Serialize)]
pub struct UserDeviceKeysResponse {
    pub public_key : String,
    pub device_id : Uuid
}
