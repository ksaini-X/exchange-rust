use once_cell::sync::Lazy;
use redis::{Client, Commands, RedisResult};
use serde::Serialize;
use std::sync::Mutex;

use crate::types::redis::{
    incoming_message_to_engine::IncomingMessageFromEngine,
    outgoing_message_to_engine::OutgoingMessageToEngine,
};

#[derive(Serialize)]
pub struct Message {
    user_id: Strin,
    client_id: String,
    message: OutgoingMessageToEngine,
}

pub struct RedisManager {
    pub receiver: Client,
    pub publisher: Client,
}

static INSTANCE: Lazy<Mutex<RedisManager>> = Lazy::new(|| {
    return Mutex::new(RedisManager::new());
});

impl RedisManager {
    fn new() -> Self {
        let receiver = Client::open("redis://localhost:6379").unwrap();
        let publisher = Client::open("redis://localhost:6379").unwrap();
        RedisManager {
            receiver,
            publisher,
        }
    }

    pub fn get_instance() -> &'static Mutex<RedisManager> {
        &INSTANCE
    }

    pub fn client_id(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }

    pub async fn send_and_await(
        &self,
        user_id: String,
        message: OutgoingMessageToEngine,
    ) -> RedisResult<IncomingMessageFromEngine> {
        let mut receiver_connection = self.receiver.get_connection().unwrap();
        let mut publisher_connection = self.publisher.get_connection().unwrap();
        let client_id = self.client_id();

        let pub_sub = receiver_connection.as_pubsub();
        pub_sub.subscribe(&client_id);

        let message = Message {
            client_id: client_id.clone(),
            message,
            user_id,
        };

        let a = publisher_connection.lpush(
            "messages",
            serde_json::to_string(&message).expect("Serilization failed"),
        )?;
        let msg_from_engine = pub_sub.get_message()?;
        let payload: String = msg_from_engine.get_payload()?;

        pub_sub.unsubscribe(&client_id);

        Ok(serde_json::from_str(&payload).expect("Failed to deserilize"))
    }
}
