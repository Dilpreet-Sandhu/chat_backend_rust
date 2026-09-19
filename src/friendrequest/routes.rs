use std::sync::Arc;

use axum::{
    Router,
    routing::{post, put},
};

use crate::{
    AppState,
    friendrequest::handlers::{accept_friend_request, create_friend_request,reject_friend_request},
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/create", post(create_friend_request))
        .route("/accept", put(accept_friend_request))
        .route("/reject",put(reject_friend_request))
}
