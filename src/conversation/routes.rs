use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post, put},
};

use crate::{
    AppState, conversation::handlers::{
        add_members, create_conversation, create_conversation_key, get_conversation, get_members_of_conversation,
        get_conversation_keys
    },
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/create", post(create_conversation))
        .route("/get-conversation", get(get_conversation))
        .route("/add-member", put(add_members))
        .route(
            "/get-members/{conversation_id}",
            get(get_members_of_conversation),
        )
        .route("/create-new-conversation-key",post(create_conversation_key))
        .route("/get-conversation-keys/{conversation_id}/{device_id}",get(get_conversation_keys))
}
