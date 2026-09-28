use axum::extract::ws::{Message, WebSocket};
use futures_util::{StreamExt, stream::SplitStream};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::socket::{
    SocketHub,
    events::{ClientEvent, ServerEvent, UserInfo},
    types::Users,
};

pub async fn read_loop(
    mut receiver: SplitStream<WebSocket>,
    socket_hub: &SocketHub,
    db_pool: &Pool<Postgres>,
    user_id: Uuid,
    device_id: Uuid,
) {
    println!("read loop");

    while let Some(result) = receiver.next().await {
        let message = match result {
            Ok(r) => r,
            Err(e) => {
                println!("receiver error {:?}", e);
                break;
            }
        };

        match message {
            Message::Text(m) => {
                let event = match serde_json::from_str::<ClientEvent>(m.as_str()) {
                    Ok(event) => event,
                    Err(e) => {
                        println!("invalid event {:?}", e);
                        continue;
                    }
                };

                match event {
                    ClientEvent::Typing {
                        conversation_id,
                        is_typing,
                    } => {

                        let users = sqlx::query_as::<Postgres, Users>(
                            "SELECT user_id from MEMBERS
                                WHERE conversation_id = $1",
                        )
                        .bind(conversation_id)
                        .fetch_all(db_pool)
                        .await;

                        let users = match users {
                            Ok(u) => u,
                            Err(e) => {

                                println!("{}",e.to_string());
                                let event = ServerEvent::Error { message: "internal server error".to_string() };
                                socket_hub.send_to_device(user_id, device_id, event).await;
                                continue;

                            }
                        };

                        for user in users {

                            let event = ServerEvent::TypingReceived { conversation_id, is_typing };

                            socket_hub.send_to_user(user.user_id, event).await;

                        } 

                    }
                    _ => {}
                }
            }
            Message::Close(f) => {
                println!("client closed socket {:?}", f);

                break;
            }
            _ => {}
        }
    }
}
