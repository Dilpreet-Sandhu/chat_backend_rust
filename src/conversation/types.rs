use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;



#[derive(Deserialize)]
pub struct CreateConversationInput {

    pub conversation_name : String,
    pub conversation_avatar : String,
    pub creator_id : uuid::Uuid,
    pub conversation_type : ConversationType

}

#[derive(Deserialize)]
pub struct ConversatonInput {

     pub conversation_name : String,
    pub conversation_avatar : String,
    pub conversation_type : String

}

#[derive(Deserialize)]
pub struct AddMemberInput {
    pub member_id : String
}

#[derive(Debug, Clone, Deserialize, sqlx::Type)]
#[sqlx(type_name = "conversation_type", rename_all = "lowercase")]
pub enum ConversationType {
    Direct,
    Group
}


#[derive(Deserialize)]
pub struct GetMembersInput {
    pub conversation_id : String
}

#[derive(Serialize,FromRow)]
pub struct MemberData {
    pub id : uuid::Uuid,
    pub username : String,
    pub avatar : String
}


#[derive(Deserialize)]
pub struct CreateNewConversationKeyType {

   pub conversation_id : String,
   pub key_version : i32,
   pub keys : Vec<DeviceEncryptionKey>
}

#[derive(Deserialize)]
pub struct DeviceEncryptionKey {
    pub device_id : String,
    pub encrypted_key : String
}
pub struct DeviceEncryptionKeyService {
    pub device_id : Uuid,
    pub encrypted_key : Vec<u8>
}


pub struct CreateNewConversationKeysServiceType { 
    pub conversation_id : Uuid,
    pub key_version : i32,
    pub keys : Vec<DeviceEncryptionKeyService>
}


#[derive(Deserialize)]
pub struct GetConversationKeysType {
    pub conversation_id : Uuid,
    pub device_id : Uuid
}

#[derive(FromRow)]
pub struct ConversationKeyResponse {
    pub key_version : i32,
    pub encrypted_key : Vec<u8>
}


#[derive(Serialize)]
pub struct ConversationKeyApiResponse {
    pub key_version : i32,
    pub encrypted_key : String
}