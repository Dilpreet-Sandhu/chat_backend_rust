use axum::{
    Json,
    extract::{Path, State},
};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

use crate::{
    AppState,
    error::AppError,
    types::ApiResponse,
    user::{
        service::UserService,
        types::{
            CreateNewDeviceServiceType, CreateNewDeviceType, GetPublicKeyServiceType,
            GetPublicKeyType, UserDeviceKeysResponse,
        },
    },
};

pub async fn create_new_device(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateNewDeviceType>,
) -> Result<ApiResponse<()>, AppError> {
    let user_id =
        Uuid::from_str(&input.user_id).map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let public_key_bytes = STANDARD
        .decode(&input.public_key)
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    UserService::create_new_device(
        CreateNewDeviceServiceType {
            user_id,
            public_key: public_key_bytes,
        },
        &state.db_pool,
    )
    .await?;

    Ok(ApiResponse {
        data: None,
        message: "created new device succefsully".to_string(),
    })
}

pub async fn get_public_key(
    State(state): State<Arc<AppState>>,
    Path(body): Path<GetPublicKeyType>,
) -> Result<ApiResponse<Vec<UserDeviceKeysResponse>>, AppError> {
    let user_id =
        Uuid::from_str(&body.user_id).map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let input = GetPublicKeyServiceType { user_id };

    let res = UserService::get_public_key(input, &state.db_pool).await?;

    Ok(ApiResponse {
        data: Some(res),
        message: "fetched keys succesfully".to_string(),
    })
}
