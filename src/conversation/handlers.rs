use std::{str::FromStr, sync::Arc};

use axum::{
    Extension, Json,
    extract::{Path, State},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use uuid::Uuid;

use crate::{
    AppState, auth::User, conversation::{
        model::Conversation, service::ConversationService, types::{
            AddMemberInput, ConversationKeyApiResponse, ConversationType, ConversatonInput, CreateConversationInput, CreateNewConversationKeyType, CreateNewConversationKeysServiceType, DeviceEncryptionKeyService, GetConversationKeysType, GetMembersInput, MemberData,
        },
    }, error::AppError, types::ApiResponse,
};

pub async fn create_conversation(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<Arc<User>>,
    Json(body): Json<ConversatonInput>,
) -> Result<ApiResponse<()>, AppError> {
    let conversation_type = match body.conversation_type.as_str() {
        "direct" => ConversationType::Direct,
        "group" => ConversationType::Group,
        _ => {
            return Err(AppError::BadRequest(String::from(
                "this type is not allowed",
            )));
        }
    };

    let input = CreateConversationInput {
        creator_id: user.id,
        conversation_name: body.conversation_name,
        conversation_avatar: body.conversation_avatar,
        conversation_type,
    };

    ConversationService::create_conversation(input, &state.db_pool).await?;

    Ok(ApiResponse {
        data: None,
        message: String::from("created conversation succesfully"),
    })
}

pub async fn add_members(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<Arc<User>>,
    Json(body): Json<AddMemberInput>,
) -> Result<ApiResponse<()>, AppError> {
    ConversationService::add_member(body, user.id, &state.db_pool).await?;

    Ok(ApiResponse {
        data: None,
        message: String::from("added member succesfully"),
    })
}

pub async fn get_members_of_conversation(
    State(state): State<Arc<AppState>>,
    Extension(_user): Extension<Arc<User>>,
    Path(body): Path<GetMembersInput>,
) -> Result<ApiResponse<Vec<MemberData>>, AppError> {
    let data = ConversationService::get_members(body, &state.db_pool).await?;

    Ok(ApiResponse {
        data: Some(data),
        message: String::from("fetched members of a chat succesfully"),
    })
}

pub async fn get_conversation(
    State(state): State<Arc<AppState>>,
    Extension(_user): Extension<Arc<User>>,
    Path(path): Path<GetMembersInput>,
) -> Result<ApiResponse<Conversation>, AppError> {
    let data: Conversation = ConversationService::get_conversation(path, &state.db_pool).await?;

    Ok(ApiResponse {
        data: Some(data),
        message: String::from("fetched conversation succesfully"),
    })
}

pub async fn create_conversation_key(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateNewConversationKeyType>,
) -> Result<ApiResponse<()>, AppError> {
    let conversation_id = Uuid::from_str(&body.conversation_id)
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    

    let mut keys: Vec<DeviceEncryptionKeyService> = Vec::new();

    for item in body.keys {
        let device_id = Uuid::from_str(&item.device_id)
            .map_err(|_| AppError::BadRequest("invalid encrypted key".to_string()))?;

        let encrypted_key = STANDARD
            .decode(item.encrypted_key)
            .map_err(|_| AppError::BadRequest("invalid encrypted key".to_string()))?;

        keys.push(DeviceEncryptionKeyService {
            device_id,
            encrypted_key,
        });
    }

    ConversationService::create_new_conversation_key(
        CreateNewConversationKeysServiceType {
            conversation_id,
            key_version: body.key_version,
            keys,
        },
        &state.db_pool,
    )
    .await?;

    Ok(ApiResponse {
        data: None,
        message: "inserted conversation key succesfully".to_string(),
    })
}


pub async fn get_conversation_keys(
    State(state) : State<Arc<AppState>>,
    Path(input) : Path<GetConversationKeysType>
) -> Result<ApiResponse<Vec<ConversationKeyApiResponse>>,AppError> {


    let keys = ConversationService::get_conversation_keys(input, &state.db_pool).await?;


    Ok(ApiResponse {
        data : Some(keys),
        message : "fetched conversation keys succesfully".to_string()
    })

}