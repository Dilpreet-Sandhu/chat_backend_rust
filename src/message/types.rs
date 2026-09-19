use uuid::Uuid;




pub struct SendMessageInput {
    pub content : String,
    pub sender_id : Uuid,
    pub conversation_id : Uuid
}

pub struct UpdateMessageInput {
    pub message_id : Uuid,
    pub content : String
}