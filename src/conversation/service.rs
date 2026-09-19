use std::str::FromStr;

use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    conversation::{
        model::Conversation, types::{AddMemberInput, CreateConversationInput, CreateNewConversationKeysServiceType, GetMembersInput, MemberData},
    }, error::AppError,
};

pub struct ConversationService;

impl ConversationService {
    pub async fn create_conversation(
        input: CreateConversationInput,
        db_pool: &Pool<Postgres>,
    ) -> Result<(), AppError> {
        let CreateConversationInput {
            conversation_name,
            conversation_avatar,
            creator_id,
            conversation_type,
        } = input;
        let id = Uuid::new_v4();

        sqlx::query(
            "INSERT INTO conversation (id,creator_id,conversation_name,conversation_avatar,conversation_type) VALUES($1,$2,$3,$4,$5)",
        )
        .bind(id)
        .bind(creator_id)
        .bind(conversation_name)
        .bind(conversation_avatar)
        .bind(conversation_type)
        .execute(db_pool)
        .await?;


        sqlx::query(
            "INSERT INTO MEMBERS (conversation_id,user_id) VALUES($1,$2)"
        )
        .bind(id)
        .bind(creator_id)
        .execute(db_pool).await?;
        

        Ok(())
    }
    pub async fn add_member(
        input: AddMemberInput,
        user_id: Uuid,
        db_pool: &Pool<Postgres>,
    ) -> Result<(), AppError> {
        let AddMemberInput { member_id } = input;

        let conversation = sqlx::query_as::<Postgres, Conversation>(
            "SELECT * from conversation
        WHERE creator_id = $1",
        )
        .bind(user_id)
        .fetch_one(db_pool)
        .await?;

        let user_id =
            Uuid::from_str(&member_id).map_err(|e| AppError::InternalServerError(e.to_string()))?;

        sqlx::query("INSERT INTO MEMBERS (conversation_id,user_id) VALUES($1,$2)")
            .bind(conversation.id)
            .bind(user_id)
            .execute(db_pool)
            .await?;

        Ok(())
    }
    pub async fn get_members(
        input: GetMembersInput,
        db_pool: &Pool<Postgres>,
    ) -> Result<Vec<MemberData>, AppError> {
        let GetMembersInput {
            conversation_id: id,
        } = input;

        let conversation_id =
            Uuid::from_str(&id).map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let members = sqlx::query_as::<Postgres, MemberData>(
            "SELECT u.id,u.username,u.avatar
            FROM USERS u
            JOIN MEMBERS m ON m.user_id = u.id
            WHERE m.conversation_id = $1",
        )
        .bind(conversation_id)
        .fetch_all(db_pool)
        .await?;

        Ok(members)
    }
    pub async fn get_conversation(
        input: GetMembersInput,
        db_pool: &Pool<Postgres>
    ) -> Result<Conversation,AppError> {


        let id = Uuid::from_str(&input.conversation_id).map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let conversation = sqlx::query_as::<Postgres,Conversation>(
            "SELECT * FROM Conversation
            WHERE id = $1"
        ).bind(id)
        .fetch_optional(db_pool)
        .await?;

        if conversation.is_none() {
            return Err(AppError::NotFound(String::from("conversation not found")));
        }
        
        Ok(conversation.unwrap())

    }
    pub async fn create_new_conversation_key(
        input : CreateNewConversationKeysServiceType
        , db_pool: &Pool<Postgres>
    ) -> Result<(),AppError> {

        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        sqlx::query(
            "INSERT INTO CONVERSATION_KEYS (id,conversation_id,device_id,key_version,encrypted_key)
            VALUES ($1,$2,$3,$4,$5),
                    ($6,$2,$7,$4,$8)"
        ).bind(id1)
        .bind(input.conversation_id)
        .bind(input.keys[0].device_id)
        .bind(input.key_version)
        .bind(input.keys[0].encrypted_key.clone())
        .bind(id2)
        .bind(input.keys[1].device_id)
        .bind(input.keys[1].encrypted_key.clone())
        .execute(db_pool)
        .await?;

        Ok(())

    }
    
}
