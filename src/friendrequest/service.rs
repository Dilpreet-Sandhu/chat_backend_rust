use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    conversation::ConversationType,
    error::AppError,
    friendrequest::{
        model::{FriendRequest, StatusType},
        types::{
            AcceptFriendRequestServiceType, CreateFriendRequestServiceType,
            RejectFriendRequestServiceType
        },
    },
};

pub struct Service;

impl Service {
    pub async fn create_friend_request(
        input: CreateFriendRequestServiceType,
        db_pool: &Pool<Postgres>,
    ) -> Result<(), AppError> {
        let CreateFriendRequestServiceType { user_id, my_id } = input;
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO friendrequest (id,sender_id,receiver_id) VALUES($1,$2,$3)")
            .bind(id)
            .bind(my_id)
            .bind(user_id)
            .execute(db_pool)
            .await?;

        Ok(())
    }
    pub async fn accept_friend_request(
        input: AcceptFriendRequestServiceType,
        db_pool: &Pool<Postgres>,
    ) -> Result<(), AppError> {
        let mut tx = db_pool.begin().await?;

        let request = sqlx::query_as::<Postgres, FriendRequest>(
            "SELECT * FROM friendrequest
            WHERE id = $1
            FOR UPDATE",
        )
        .bind(input.request_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::NotFound("request not found".to_string()))?;

        match request.status {
            StatusType::REJECTED => {
                return Err(AppError::Conflict(
                    "request is already rejected".to_string(),
                ));
            }
            StatusType::ACCEPTED => {
                return Err(AppError::Conflict(
                    "request is already accepted".to_string(),
                ));
            }
            _ => {}
        };

        sqlx::query(
            "UPDATE friendrequest
                SET status = $1
                WHERE id = $2",
        )
        .bind(StatusType::ACCEPTED)
        .bind(request.id)
        .execute(&mut *tx)
        .await?;

        let conversation_id: Uuid = sqlx::query_scalar(
            "INSERT INTO conversation (id,conversation_type)
            VALUES ($1,$2)
            RETURNING id",
        )
        .bind(Uuid::new_v4())
        .bind(ConversationType::Direct)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query(
            "INSERT INTO MEMBERS
                  (conversation_id,user_id) VALUES
                  ($1,$2),
                  ($1,$3)",
        )
        .bind(conversation_id)
        .bind(request.sender_id)
        .bind(request.receiver_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
    pub async fn reject_friend_request(
        input: RejectFriendRequestServiceType,
        db_pool: &Pool<Postgres>,
    ) -> Result<(), AppError> {
        let RejectFriendRequestServiceType { request_id } = input;

        let request =
            sqlx::query_as::<Postgres, FriendRequest>("SELECT * FROM FriendRequest WHERE id = $1")
                .bind(request_id)
                .fetch_optional(db_pool)
                .await?
                .ok_or_else(|| AppError::NotFound("request not found".to_string()))?;

        match request.status {
            StatusType::ACCEPTED => {
                return Err(AppError::Conflict("request already accepted".to_string()));
            }
            StatusType::REJECTED => {
                return Err(AppError::Conflict("request already rejected".to_string()));
            }
            _ => {}
        };

        sqlx::query(
            "UPDATE friendrequest
                SET status = $1
                WHERE id = $2",
        )
        .bind(StatusType::REJECTED)
        .bind(request.id)
        .execute(db_pool)
        .await?;

        Ok(())
    }
}
