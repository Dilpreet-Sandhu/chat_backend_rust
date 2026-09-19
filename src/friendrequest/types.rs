use serde::{Deserialize};
use uuid::Uuid;



#[derive(Deserialize)]
pub struct CreateFriendRequestType {
    
    pub user_id : String

}

pub struct CreateFriendRequestServiceType {
    pub user_id : Uuid,
    pub my_id : Uuid
}

#[derive(Deserialize)]
pub struct AcceptFriendRequestType {
    pub id : String
}
pub struct AcceptFriendRequestServiceType {
    pub request_id : Uuid
}

pub struct RejectFriendRequestServiceType {
    pub request_id : Uuid
}
#[derive(Deserialize)]
pub struct RejectFriendRequestType {
    pub id : String
}