use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone,Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "conversation_type", rename_all = "lowercase")]
pub enum ConversationType {
    Direct,
    Group
}

#[derive(Serialize,Deserialize,FromRow,Clone)]
pub struct Conversation {

    pub id : Uuid,
    pub creator_id : Uuid,
    pub conversation_name : String,
    pub conversation_avatar : String,
    pub conversation_type : ConversationType

}


#[derive(Serialize,FromRow,Clone)]
pub struct ConversationKey {
    pub id : Uuid,
    pub conversation_id : Uuid,
    pub device_id : Uuid,
    pub key_version : u32,
    pub encrypted_key : Vec<u8>,
    pub created_at : DateTime<Utc>
}