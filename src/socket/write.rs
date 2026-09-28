use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, stream::SplitSink};
use tokio::sync::mpsc::Receiver;

use crate::socket::{SocketHub, events::ServerEvent};



pub async fn write_loop(mut rx : Receiver<ServerEvent>,mut sender : SplitSink<WebSocket,Message>) {

    while let Some(event) = rx.recv().await {

            println!("outgoing loop {:?}",event);

            let json = match serde_json::to_string(&event) {
                Ok(m) => m,
                Err(e) => {

                    println!("serializating error {}",e);

                    continue;

                }
            };


            let message = Message::Text(json.into());


            if let Err(err) = sender.send(message).await {
                println!("websocket error {:?}",err);
                break;
            }


    }

}