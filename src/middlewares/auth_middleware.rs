use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use sqlx::Postgres;
use uuid::Uuid;

use crate::{
    AppState,
    auth::{User, extra::decode_access_token},
    error::AppError,
};

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = request
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized(String::from("you are not authorized")))?;

    let decoded_token = decode_access_token(token, &state.access_token_secret)
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let user_id = Uuid::parse_str(&decoded_token.sub)
    .map_err(|v| AppError::InternalServerError(v.to_string()))?;

    let user = sqlx::query_as::<Postgres, User>("SELECT * FROM USERS WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&state.db_pool)
        .await?
        .ok_or(AppError::NotFound(String::from("user not found")))?;

    request.extensions_mut().insert(Arc::new(user));

    Ok(next.run(request).await)
}
