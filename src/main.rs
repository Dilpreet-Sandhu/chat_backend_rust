use std::{collections::HashMap, sync::Arc};

use axum::{Router, middleware, routing::{any,get}};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tokio::{
    net::TcpListener,
    sync::{RwLock, mpsc::Sender},
};
use uuid::Uuid;

use crate::{
    middlewares::auth_middleware, socket::{SocketHub, events::ServerEvent, ws_handler},
};

mod auth;
mod conversation;
mod error;
mod friendrequest;
mod message;
pub mod middlewares;
mod socket;
mod types;
mod user;
#[derive(Clone)]
pub struct AppState {
    db_pool: Pool<Postgres>,
    access_token_secret: String,
    socket_hub: SocketHub
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("database url is required");
    let access_token_secret =
        std::env::var("ACCESS_TOKEN_SECRET").expect("access token secret is required");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let socket_hub = SocketHub::new();

    let app_state = Arc::new(AppState {
        db_pool: pool,
        access_token_secret,
        socket_hub,
    });

    let public_router = Router::new().nest("/auth", auth::routes());

    let protected_rotues = Router::new()
        .nest("/conversation", conversation::routes())
        .nest("/friendreq", friendrequest::routes())
        .nest("/message", message::routes())
        .nest("/user", user::routes())
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ));

    let router = Router::new()
        .merge(public_router)
        .merge(protected_rotues)
        .route("/ws", any(ws_handler))
        .with_state::<()>(app_state);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind");

    println!("server started");

    axum::serve(listener, router).await.expect("server failed");

    Ok(())
}
