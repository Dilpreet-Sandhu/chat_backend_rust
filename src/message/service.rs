use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{error::AppError, message::{model::Message, types::{SendMessageInput, UpdateMessageInput}}};



pub struct MessageService;


impl MessageService {
    pub async fn send_message(
        input : SendMessageInput,
        db_pool : &Pool<Postgres>
    ) -> Result<Message,AppError> {

        let SendMessageInput {content,sender_id,conversation_id} = input;

        let id = Uuid::new_v4();
        let message =sqlx::query_as::<Postgres,Message>(
            "INSERT INTO MESSAGE (id,conversation_id,content,created_at,sender_id)
            VALUES ($1,$2,$3,NOW(),$4)
            RETURNS *"
        ).bind(id)
        .bind(conversation_id)
        .bind(content)
        .bind(sender_id)
        .fetch_one(db_pool)
        .await?;
        

        Ok(message)
    }
    pub async fn update_message(
        input : UpdateMessageInput,
        db_pool : &Pool<Postgres>
    ) -> Result<Message,AppError> {


        let UpdateMessageInput { message_id, content } = input;

        let old_message = sqlx::query_as::<Postgres,Message>(
            "SELECT * FROM MESSAGE WHERE id = $1"
        ).bind(message_id)
        .fetch_optional(db_pool)
        .await?
        .ok_or_else(|| AppError::NotFound("message not found".to_string()))?;


        let new_message = sqlx::query_as::<Postgres,Message>(
            "UPDATE MESSAGE 
            SET content = $1
            WHERE id = $2
            RETURNS *"
        ).bind(content)
        .bind(old_message.id)
        .fetch_one(db_pool)
        .await?;


        Ok(new_message)

    }
}