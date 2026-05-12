use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::orderbook::{Fill, Order, OrderStatus};
#[derive(Debug, Deserialize, Serialize)]
pub enum IncomingMessageFromEngine {
    Depth {
        market: String,
        asks: Vec<(Decimal, Decimal)>,
        bids: Vec<(Decimal, Decimal)>,
    },
    OrderPlaced {
        order_id: String,
        fills: Vec<Fill>,
        executed_quantity: Decimal,
        order_status: OrderStatus,
    },
    OrderCancelled {
        order_id: String,
        executed_qty: Decimal,
        remaining_qty: Decimal,
    },

    OpenOrders {
        orders: Vec<Order>,
    },
    Error {
        message: String,
    },
}
