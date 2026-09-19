use std::sync::Arc;

use axum::{
    Router, routing::{post, put,get}
};

use crate::{AppState, auth::handlers::{create_user, login_user,get_users,get_user,get_me}};


pub fn routes() -> Router<Arc<AppState>> {

    Router::new()
    .route("/register",post(create_user))
    .route("/login",put(login_user))
    .route("/users",get(get_users))
    .route("/me",get(get_me)) 
    .route("/user/{id}",get(get_user))
}