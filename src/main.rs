use std::{sync::Arc};

use axum::{Router, middleware};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tokio::net::TcpListener;

use crate::middlewares::auth_middleware;

mod auth;
mod types;
mod error;
mod friendrequest;
mod conversation;
mod message;
mod user;
pub mod middlewares;
#[derive(Clone)]
pub struct AppState {

    db_pool : Pool<Postgres>,
    access_token_secret : String
}

#[tokio::main]
async fn main() -> Result<(),sqlx::Error> {

    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("database url is required");
    let access_token_secret = std::env::var("ACCESS_TOKEN_SECRET").expect("access token secret is required");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&db_url)
    .await?;

    let app_state = Arc::new(AppState {
        db_pool : pool,
        access_token_secret
    });

    let public_router = Router::new()
    .nest("/auth",auth::routes());

    let protected_rotues = Router::new()
    .nest("/conversation",conversation::routes())
    .nest("/friendreq",friendrequest::routes())
    .nest("/message",message::routes())
    .nest("/user",user::routes())
    .route_layer(
        middleware::from_fn_with_state(app_state.clone(), auth_middleware)
    );

    let router = Router::new()
    .merge(public_router)
    .merge(protected_rotues)
    .with_state::<()>(app_state);


   let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind");

    println!("server started"); 


    axum::serve(listener,router).await.expect("server failed");


    Ok(())

    
}