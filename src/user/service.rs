use base64::{Engine, engine::general_purpose::STANDARD};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{error::AppError, user::{model::UserDevice, types::{CreateNewDeviceServiceType, GetPublicKeyServiceType, UserDeviceKeysResponse, UserDevicePublicKeys}}};





pub struct UserService;


impl UserService {
    pub async fn create_new_device(
        input : CreateNewDeviceServiceType,
        db_pool : &Pool<Postgres>
    ) -> Result<(),AppError> {

        let id = Uuid::new_v4();

        sqlx::query(
            "INSERT INTO USER_DEVICE (id,user_id,public_key,created_at,last_seen_at)
            VALUES ($1,$2,$3,NOW(),NOW())"
        ).bind(id)
        .bind(input.user_id)
        .bind(input.public_key)
        .execute(db_pool)
        .await?;



        Ok(())
    }
    pub async fn get_public_key(
        input : GetPublicKeyServiceType,
        db_pool : &Pool<Postgres>
    ) -> Result<Vec<UserDeviceKeysResponse>,AppError> {

        
        let devices : Vec<UserDevicePublicKeys> = sqlx::query_as::<Postgres,UserDevicePublicKeys>(
            "SELECT id as device_id,public_key FROM USER_DEVICE 
            WHERE user_id = $1"
        ).bind(input.user_id)
        .fetch_all(db_pool)
        .await?;


        let response : Vec<UserDeviceKeysResponse> = devices
        .into_iter()
        .map(|e|  UserDeviceKeysResponse {

            device_id : e.device_id,
            public_key : STANDARD.encode(e.public_key)

        }).collect();


        Ok(response)
    }
}