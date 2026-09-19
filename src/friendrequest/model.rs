use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone,Serialize, Deserialize, sqlx::Type,PartialEq)]
#[sqlx(type_name = "status_type")]
pub enum StatusType {
    PENDING,
    ACCEPTED,
    REJECTED
}


#[derive(Serialize,Deserialize,FromRow,Clone)]
pub struct FriendRequest {

    pub id : Uuid,
    pub sender_id : Uuid,
    pub receiver_id : Uuid,
    pub status : StatusType,
    pub created_at : DateTime<Utc>
}

