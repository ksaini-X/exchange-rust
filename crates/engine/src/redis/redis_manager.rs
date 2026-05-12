use std::sync::Mutex;

use once_cell::sync::Lazy;
use redis::{Client, Commands, RedisResult};

use crate::types::api::outgoing_to_api::OutgoingMessageToAPI;

#[derive(Debug)]
pub struct RedisManager {
    redis_client: Client,
}

static INSTANCE: Lazy<Mutex<RedisManager>> = Lazy::new(|| {
    return Mutex::new(RedisManager::new());
});

impl RedisManager {
    pub fn new() -> Self {
        let redis_url = "";
        let redis_client = Client::open(redis_url).expect("Redis Client creation Failed");
        Self { redis_client }
    }

    pub fn get_instance() -> &'static Lazy<Mutex<RedisManager>> {
        &INSTANCE
    }

    pub fn publish_message(
        &self,
        client_id: &str,
        message: OutgoingMessageToAPI,
    ) -> RedisResult<()> {
        let enchanced_messge = match message {
            OutgoingMessageToAPI::OrderPlaced {
                order_id,
                fills,
                executed_quantity,
                order_status,
            } => {
                serde_json::json!({
                    "type":"ORDER_PLACED",
                    "payload":{
                        "order_id":order_id,
                        "fills":fills,
                        "executed_quantity":executed_quantity,
                        "order_status":order_status,
                    }
                })
            }
            OutgoingMessageToAPI::Depth { market, asks, bids } => {
                serde_json::json!({
                    "type":"DEPTH",
                    "payload":{
                        "asks":asks,
                        "bids":bids,
                    }
                })
            }
            OutgoingMessageToAPI::Error { message } => {
                serde_json::json!({
                    "type":"ERROR",
                    "payload":{
                        "error":message,
                    }
                })
            }
            OutgoingMessageToAPI::OpenOrders { orders } => {
                serde_json::json!({
                    "type":"ORDER_PLACED",
                    "payload":{
                        "orders":orders,
                    }
                })
            }
            OutgoingMessageToAPI::OrderCancelled {
                order_id,
                executed_qty,
                remaining_qty,
            } => {
                serde_json::json!({
                    "type":"ORDER_PLACED",
                    "payload":{
                        "order_id":order_id,
                        "executed_qty":executed_qty,
                        "remaining_qty":remaining_qty,
                    }
                })
            }
        };
        let string_enchanced_message = serde_json::to_string(&enchanced_messge).unwrap();

        let mut connection = self.redis_client.get_connection()?;
        let publisher_response = connection.publish(client_id, string_enchanced_message);
        match publisher_response {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }
}
