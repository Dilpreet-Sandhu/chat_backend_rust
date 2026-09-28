use std::{collections::HashMap, sync::Arc};

use tokio::sync::{RwLock, mpsc::Sender};
use uuid::Uuid;

use crate::socket::events::ServerEvent;





#[derive(Clone)]
pub struct SocketHub {
    pub connections : Arc<
        RwLock<
            HashMap<
                Uuid,
                HashMap<
                Uuid,
                Sender<ServerEvent>
                >
            >
        >
    >
}

//user_id -> {
//
//device_id -> tx
//


impl SocketHub {
    pub fn new() -> Self {

        SocketHub {
            connections : Arc::new(RwLock::new(HashMap::new()))
        }

    }
    pub async fn connect(
        &self,
        user_id : Uuid,
        device_id : Uuid,
        tx : Sender<ServerEvent>
    )  {

        let mut connections = self.connections.write().await;

        connections.entry(user_id)
        .or_default()
        .insert(device_id,tx);

    }
    pub async fn disconnect(
        &self,
        user_id : Uuid,
        device_id : Uuid
    ) {

        let mut connections = self.connections.write().await;

        if let Some(devices) = connections.get_mut(&user_id) {

            devices.remove(&device_id);

            if devices.is_empty() {
                connections.remove(&user_id);
            }

        }

    }
    pub async fn send_to_device(
        &self,
        user_id : Uuid,
        device_id : Uuid,
        event : ServerEvent
    ) {

        let tx = {

            let connections = self.connections.read().await;

            connections.get(&user_id)
            .and_then(|devices| devices.get(&device_id))
            .cloned()

        };


        if let Some(tx) = tx {
            let _ = tx.send(event).await;
        }
        

    }
    pub async fn send_to_user(
        &self,
        user_id : Uuid,
        event : ServerEvent
    ) {

        let tx = {

            let connections = self.connections.read().await;

            connections.get(&user_id)
            .map(|devices| {

                devices
                .values()
                .cloned()
                .collect::<Vec<_>>()
                

            }).unwrap_or_default()

        };


        for t in tx {
            let _ = t.send(event.clone()).await;
        }


    }
} 