use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize,Debug)]
pub struct UserInfo {
    pub user_id : Uuid,
    pub device_id : Uuid
}


#[derive(Debug,Deserialize)]
#[serde(
    tag="type",
    content="data",
    rename_all="snake_case"
)]
pub enum ClientEvent {

    Ping,
    Typing {
        conversation_id : Uuid,
        is_typing : bool,
    },
    Something

}

#[derive(Debug,Serialize,Clone)]
#[serde(
    tag="type",
    content="data",
    rename_all="snake_case"
)]
pub enum ServerEvent {

    Pong,
    TypingReceived {
        conversation_id : Uuid,
        is_typing : bool
    },
    Error {
        message : String
    }

}