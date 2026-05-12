use rust_decimal::Decimal;
use serde::Deserialize;

use crate::orderbook::Side;
#[derive(Debug, Deserialize)]

pub enum OutgoingMessageToEngine {
    CreateOrder {
        market: String,
        price: Decimal,
        quantity: Decimal,
        side: Side,
    },
    CancelOrder {
        order_id: String,
        user_id: String,
    },
    GetDepth {
        market: String,
    },
    GetOpenOrders {
        user_id: String,
        market: String,
    },
}

#[derive(Serialize)]
pub struct CreateOrderData {
    pub user_id: String,
    pub market: String,
    pub quantity: Decimal,
    pub price: Decimal,
    pub side: Side,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Side {
    Bid,
    Ask,
}
