use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::{Uuid};




#[derive(Serialize,Deserialize,FromRow,Clone)]
pub struct Message {
    pub id : Uuid,
    pub conversation_id : Uuid,
    pub content : String,
    pub created_at : DateTime<Utc>,
    pub sender_id : Uuid
}


