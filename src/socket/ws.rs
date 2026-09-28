use std::sync::Arc;

use axum::{
    extract::{Query, State, WebSocketUpgrade, ws::WebSocket}, response::{IntoResponse},
};
use futures_util::StreamExt;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::socket::read::read_loop;
use crate::socket::write::write_loop;
use crate::{
    AppState,

    socket::{ events::ServerEvent, types::Params},
};

pub async fn ws_handler(
    Query(query): Query<Params>,
    ws: WebSocketUpgrade,
    State(state) : State<Arc<AppState>>,
) -> impl IntoResponse {
    let Params { user_id, device_id } = query;

    ws.on_upgrade(move |socket| handle_socket(socket, state, user_id, device_id))
}

pub async fn handle_socket(
    socket: WebSocket,
    state: Arc<AppState>,
    user_id: Uuid,
    device_id: Uuid,
) {
    let (sender, receiver) = socket.split();

    let (tx, rx) = mpsc::channel::<ServerEvent>(128);

    state.socket_hub.connect(user_id, device_id, tx.clone()).await;

    tokio::select! {
        _ = read_loop(receiver,&state.socket_hub,&state.db_pool,user_id,device_id) => {
            println!("reader terminated connection");
        }
        _ = write_loop(rx,sender) => {
            println!("writer terminated connection");
        }
    }

    state.socket_hub.disconnect(user_id, device_id).await;
}

// while let Some(res) = socket.recv().await {
//     let message = match res {
//         Ok(r) => r,
//         Err(e) => {
//             println!("socket error {}", e);
//             break;
//         }
//     };

//     match message {
//         Message::Text(m) => {
//             let event =
//             serde_json::from_str::<ClientEvent>(m.as_str());

//             let event = match event {
//                 Ok(e) => e,
//                 Err(e) => {

//                     println!("invalid event {}",e);
//                     continue;

//                 }
//             };

//             match event {

//                 ClientEvent::Ping => {

//                     let res = ServerEvent::Pong;

//                     let json = serde_json::to_string(
//                         &res
//                     ).unwrap();

//                     if socket.send(Message::Text(json.into())).await.is_err() {
//                         break;
//                     }

//                 },
//                 ClientEvent::Typing { conversation_id, is_typing } => {

//                     println!("{} {}",conversation_id,is_typing);

//                     let response = ServerEvent::TypingReceived { conversation_id, is_typing };

//                     let json = serde_json::to_string(&response).unwrap();

//                     if socket.send(Message::Text(json.into())).await.is_err() {
//                         break;
//                     }

//                 },
//                 _ => {}

//             }

//         }

//         _ => {}
//     }
// }
