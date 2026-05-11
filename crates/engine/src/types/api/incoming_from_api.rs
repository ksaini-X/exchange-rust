use rust_decimal::Decimal;
use serde::Deserialize;

use crate::orderbook::Side;
#[derive(Debug, Deserialize)]

pub enum IncomingMessageFromAPI {
    CreateOrder {
        market: String,
        price: Decimal,
        quantity: Decimal,
        side: Side,
    },
    CancelOrder {
        market: String,
        order_id: String,
        user_id: String,
    },
    GetDepth {
        market: String,
    },
}
