use std::collections::HashMap;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

use crate::{
    orderbook::{Fill, Order, OrderStatus, Orderbook, Side},
    types::api::incoming_from_api::IncomingMessageFromAPI,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserBalance {
    available: Decimal,
    locked: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    SOL,
    USDC,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Engine {
    pub orderbooks: Vec<Orderbook>,
    pub users: HashMap<String, HashMap<Currency, UserBalance>>,
}

impl Engine {
    pub fn new() -> Self {
        let def_orderbook = Orderbook::new("SOL/USDC".to_string());
        Self {
            orderbooks: vec![def_orderbook],
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user_id: String) -> Result<(), String> {
        if self.users.contains_key(&user_id) {
            Err("User exists".to_string())
        } else {
            let mut def_balance: HashMap<Currency, UserBalance> = HashMap::new();

            def_balance.insert(
                Currency::USDC,
                UserBalance {
                    available: dec!(10_000_000),
                    locked: dec!(0),
                },
            );
            self.users.insert(user_id, def_balance);
            Ok(())
        }
    }

    pub fn process(&mut self, message: IncomingMessageFromAPI, client_id: String, user_id: String) {
        match IncomingMessageFromAPI {
            IncomingMessageFromAPI::CreateOrder {
                market,
                price,
                quantity,
                side,
            } => {
                let respone = self.create_order(&market, price, quantity, side, user_id);
                match respone {
                    Err() => {}
                    Ok((order_id, fills, executed_quantity)) => {}
                }
            }
            IncomingMessageFromAPI::CancelOrder {
                market,
                order_id,
                user_id,
            } => {}
            IncomingMessageFromAPI::GetDepth { market } => {}
        }
    }

    pub fn create_order(
        &mut self,
        market: &str,
        price: Decimal,
        quantity: Decimal,
        side: Side,
        user_id: String,
    ) -> Result<(String, Vec<Fill>, Decimal), String> {
        let orderbook = self
            .orderbooks
            .iter_mut()
            .find(|makret| makret.ticker() == market)
            .unwrap();

        let order_id = uuid::Uuid::new_v4().to_string();
        let order = Order {
            order_id: order_id.clone(),
            user_id,
            price,
            quantity,
            filled_quantity: dec!(0),
            side,
            status: OrderStatus::Pending,
            timestamp: chrono::Utc::now().timestamp(),
        };

        let (fills, executed_quantity) = orderbook.add_order(order).unwrap();

        Ok((order_id, fills, executed_quantity))
    }
}
