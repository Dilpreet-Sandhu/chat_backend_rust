use std::sync::Arc;

use axum::{
    Extension, Json, extract::{Path, State},
};

use crate::{
    AppState, auth::{
        User, dto::{UserCreateRequest, UserLoginRequest, UserLoginResponse, UserResponse}, service::AuthService,
    }, error::AppError, types::ApiResponse,
};

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(body): Json<UserCreateRequest>,
) -> Result<ApiResponse<()>, AppError> {
    AuthService::create_user(
        (body.username, body.email, body.password, body.avatar),
        &state.db_pool,
    )
    .await?;

    Ok(ApiResponse::<()> {
        message: String::from("user created succesfully"),
        data: None,
    })
}

pub async fn login_user(
    State(state): State<Arc<AppState>>,
    Json(body): Json<UserLoginRequest>,
) -> Result<ApiResponse<UserLoginResponse>, AppError> {
    let response = AuthService::login_user(
        (body.identifier, body.password),
        &state.db_pool,
        &state.access_token_secret,
    )
    .await?;

    Ok(ApiResponse {
        data: Some(response),
        message: String::from("user logged in succesfully"),
    })
}

pub async fn get_users(
    State(state): State<Arc<AppState>>,
) -> Result<ApiResponse<Vec<UserResponse>>, AppError> {
    let users = AuthService::get_users(&state.db_pool).await?;

    Ok(ApiResponse {
        data: Some(users),
        message: String::from("users fetched succesfully"),
    })
}

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<ApiResponse<UserResponse>, AppError> {
    let user = AuthService::get_user(id, &state.db_pool).await?;

    Ok(ApiResponse {
        data: Some(user),
        message: String::from("user fetched succesfully"),
    })
}

pub async fn get_me(
    Extension(user) : Extension<Arc<User>>
) -> Result<ApiResponse<UserResponse>, AppError> {
    let user: UserResponse = UserResponse {
        id: user.id,
        email: user.email.clone(),
        username: user.username.clone(),
        avatar : user.avatar.clone()
    };

    Ok(ApiResponse {
        data: Some(user),
        message: String::from("fetched user succesfully"),
    })
}
