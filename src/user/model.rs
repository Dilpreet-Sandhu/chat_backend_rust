use chrono::{DateTime, Utc};
use uuid::Uuid;



pub struct UserDevice {
    pub id : Uuid,
    pub user_id : Uuid,
    pub public_key : Vec<u8>,
    pub created_at : DateTime<Utc>,
    pub last_seen_at : DateTime<Utc>
}