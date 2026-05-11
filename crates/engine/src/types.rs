use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    order_id: u64,
    user_id: u64,
    price: Decimal,
    quantity: Decimal,
    filled_quantity: Decimal,
    side: Side,
    status: OrderStatus,
    timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fill {
    trade_id: u64,
    maker_order_id: u64,
    taker_order_id: u64,
    price: Decimal,
    quantity: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Filled,
    PartiallyFilled,
    Cancelled,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Orderbook {
    market: String,
    pub bids: BTreeMap<Decimal, Vec<Order>>, // {99.00 : [abc,red,aes...], 98 : [das,ewa,dra....]}
    pub asks: BTreeMap<Decimal, Vec<Order>>, // {100.00 : [abc,red,aes...], 101 : [das,ewa,dra....]}
    pub orders: HashMap<String, Order>,      // {abc : Order{}, def:Order{}, ijk :Order{}....}
    pub last_trade_id: u64,
    pub last_traded_price: Decimal,
    pub last_snapshot_timestamp: i64,
}

impl Orderbook {
    pub fn new(market: String) -> Self {
        Self {
            market,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            orders: HashMap::new(),
            last_trade_id: 0,
            last_traded_price: dec!(0),
            last_snapshot_timestamp: 0,
        }
    }

    pub fn snapshot(&mut self) -> Orderbook {
        let ts = Utc::now().timestamp();
        self.last_snapshot_timestamp = ts;

        Orderbook {
            market: self.market.clone(),
            bids: self.bids.clone(),
            asks: self.asks.clone(),
            orders: self.orders.clone(),
            last_trade_id: self.last_trade_id,
            last_traded_price: self.last_traded_price,
            last_snapshot_timestamp: ts,
        }
    }

    pub fn add_order(&mut self, mut order: Order) -> Result<(Vec<Fill>, Decimal), String> {
        order.timestamp = Utc::now().timestamp_millis();

        match order.side {
            Side::Ask => {
                let (fills, executed_quantity) = self.match_bids(&order)?;
                order.filled_quantity = executed_quantity;

                order.status = if executed_quantity == order.quantity {
                    OrderStatus::Filled
                } else if executed_quantity > dec!(0) {
                    OrderStatus::PartiallyFilled
                } else {
                    OrderStatus::Pending
                };

                if order.filled_quantity < order.quantity {
                    self.asks
                        .entry(order.price)
                        .and_modify(|orders| orders.push(order.clone()))
                        .or_insert(vec![order.clone()]);
                }

                self.orders.insert(order.order_id.to_string(), order);

                Ok((fills, executed_quantity))
            }

            Side::Bid => {
                let (fills, executed_quantity) = self.match_asks(&order)?;
                order.filled_quantity = executed_quantity;

                order.status = if executed_quantity == order.quantity {
                    OrderStatus::Filled
                } else if executed_quantity > dec!(0) {
                    OrderStatus::PartiallyFilled
                } else {
                    OrderStatus::Pending
                };

                if order.filled_quantity < order.quantity {
                    self.bids
                        .entry(order.price)
                        .and_modify(|orders| orders.push(order.clone()))
                        .or_insert(vec![order.clone()]);
                }

                self.orders.insert(order.order_id.to_string(), order);

                Ok((fills, executed_quantity))
            }
        }
    }

    pub fn match_asks(&mut self, order: &Order) -> Result<(Vec<Fill>, Decimal), String> {
        // Incoming Order {price:101, qty: 10, side:BID}

        //  asks = {                                      order.price >= ask.price     Fill | Left (Order)          Ask
        //     100.00 -> [
        //         Order{order_id:1, price:100, quantity:3}     Match                      3 | 7              {qty:3, filled:3}
        //         Order{order_id:2, price:100, quantity:4}     Match                      4 | 4              {qty:3, filled:3}
        //         ],
        //     101.00 -> [
        //          Order{order_id:2, price:101, quantity:3}     Match                      3 | 1              {qty:3, filled:3}
        //         ]
        // }

        let mut fills = Vec::new();
        let mut executed_quantity = dec!(0);

        for (price, asks) in self.asks.iter_mut() {
            if order.price < *price {
                break;
            }

            for ask in asks.iter_mut() {
                if order.price >= ask.price && executed_quantity < order.quantity {
                    let left_quantity = order.quantity - executed_quantity;
                    let filled_quantity = std::cmp::min(left_quantity, ask.quantity);
                    self.last_trade_id += 1;
                    executed_quantity += filled_quantity;

                    ask.filled_quantity += filled_quantity;
                    fills.push(Fill {
                        price: ask.price,
                        quantity: filled_quantity,
                        trade_id: self.last_trade_id,
                        maker_order_id: ask.order_id,
                        taker_order_id: order.order_id,
                    });
                }
            }
            asks.retain(|ask| ask.filled_quantity < ask.quantity);
        }
        Ok((fills, executed_quantity))
    }

    pub fn match_bids(&mut self, order: &Order) -> Result<(Vec<Fill>, Decimal), String> {
        // Incoming Order {price:98, qty: 10, side : ASK}

        //  bids = {                                      order.price <= bid.price     Fill | Left (Order)          bid
        //     100.00 -> [
        //         Order{order_id:1, price:100, quantity:3}     Match                      3 | 7              {qty:3, filled:3}
        //         Order{order_id:2, price:100, quantity:4}     Match                      4 | 4              {qty:3, filled:3}
        //         ],
        //     99.00 -> [
        //          Order{order_id:2, price:101, quantity:3}     Match                      3 | 1              {qty:3, filled:3}
        //         ]
        // }
        let mut fills = Vec::new();
        let mut executed_quantity = dec!(0);

        for (price, bids) in self.bids.iter_mut().rev() {
            if *price < order.price {
                break;
            }

            for bid in bids.iter_mut() {
                if order.price <= bid.price && executed_quantity < order.quantity {
                    let left_quantity = order.quantity - executed_quantity;
                    let filled_quantity = std::cmp::min(left_quantity, bid.quantity);
                    self.last_trade_id += 1;
                    executed_quantity += filled_quantity;

                    bid.filled_quantity += filled_quantity;
                    fills.push(Fill {
                        price: bid.price,
                        quantity: filled_quantity,
                        trade_id: self.last_trade_id,
                        maker_order_id: bid.order_id,
                        taker_order_id: order.order_id,
                    });
                }
            }
            bids.retain(|bid| bid.filled_quantity < bid.quantity);
        }
        Ok((fills, executed_quantity))
    }
}
