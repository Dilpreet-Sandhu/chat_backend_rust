use std::sync::Arc;

use axum::{Router, routing::{get, post}};

use crate::{AppState, user::handler::{create_new_device,get_public_key}};



pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
    .route("/create-new-device",post(create_new_device))
    .route("/get-public-keys/{user_id}",get(get_public_key))
}