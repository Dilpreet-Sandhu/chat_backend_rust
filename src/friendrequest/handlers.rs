use std::{str::FromStr, sync::Arc};

use axum::{Extension, Json, extract::State};
use uuid::Uuid;

use crate::{
    AppState, auth::User, error::AppError, friendrequest::{
        service::Service, types::{AcceptFriendRequestServiceType, AcceptFriendRequestType, CreateFriendRequestServiceType, CreateFriendRequestType, RejectFriendRequestServiceType, RejectFriendRequestType},
    }, types::ApiResponse,
};

pub async fn create_friend_request(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<Arc<User>>,
    Json(body): Json<CreateFriendRequestType>,
) -> Result<ApiResponse<()>, AppError> {
    let id = body.user_id;

    let my_id: Uuid = user.id;
    let user_id = Uuid::from_str(&id).map_err(|e| AppError::InternalServerError(e.to_string()))?;

    Service::create_friend_request(
        CreateFriendRequestServiceType { user_id, my_id },
        &state.db_pool,
    )
    .await?;

    Ok(ApiResponse {
        data: None,
        message: String::from("friend request created"),
    })
}


pub async fn accept_friend_request(
    State(state) : State<Arc<AppState>>,
    Extension(_user): Extension<Arc<User>>,
    Json(body) : Json<AcceptFriendRequestType>
) -> Result<ApiResponse<()>,AppError> {

    let id = body.id;

    let request_id = Uuid::from_str(&id).map_err(|e| AppError::InternalServerError(e.to_string()))?;


    let input : AcceptFriendRequestServiceType = AcceptFriendRequestServiceType {request_id};


    Service::accept_friend_request(input, &state.db_pool).await?;


    Ok(ApiResponse { data: None, message: "accepted friend request".to_string() })

}

pub async fn reject_friend_request(
    State(state) : State<Arc<AppState>>,
    Extension(_user) : Extension<Arc<User>>,
    Json(body) : Json<RejectFriendRequestType>
) -> Result<ApiResponse<()>,AppError> {

    let id = body.id;

   let request_id = Uuid::from_str(&id).map_err(|e| AppError::InternalServerError(e.to_string()))?; 


    let input : RejectFriendRequestServiceType = RejectFriendRequestServiceType { request_id };


    Service::reject_friend_request(input, &state.db_pool).await?;

    Ok(ApiResponse {
        data : None,
        message : "rejected friend request".to_string()
    })

}