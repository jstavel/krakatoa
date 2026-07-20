use crate::types::{BookSnapshot, OrderResult, OrderStatus, PriceLevel};

pub struct OrderBook {
    bids: Vec<PriceLevel>,
    order_ids: Vec<String>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            bids: Vec::with_capacity(128),
            order_ids: Vec::with_capacity(128),
        }
    }

    pub fn place_limit_buy(&mut self, order_id: String, price: u64, qty: u64) -> OrderResult {
        if price == 0 {
            return OrderResult {
                order_id,
                status: OrderStatus::Rejected,
                trades: vec![],
                book_snap: BookSnapshot {
                    bids_changed: vec![],
                    asks_changed: vec![],
                },
            };
        }

        if qty == 0 {
            return OrderResult {
                order_id,
                status: OrderStatus::Rejected,
                trades: vec![],
                book_snap: BookSnapshot {
                    bids_changed: vec![],
                    asks_changed: vec![],
                },
            };
        }

        if order_id.is_empty() {
            return OrderResult {
                order_id,
                status: OrderStatus::Rejected,
                trades: vec![],
                book_snap: BookSnapshot {
                    bids_changed: vec![],
                    asks_changed: vec![],
                },
            };
        }

        if self.order_ids.contains(&order_id) {
            return OrderResult {
                order_id,
                status: OrderStatus::Rejected,
                trades: vec![],
                book_snap: BookSnapshot {
                    bids_changed: vec![],
                    asks_changed: vec![],
                },
            };
        }

        for level in self.bids.iter_mut() {
            if level.price == price {
                level.qty += qty;
                self.order_ids.push(order_id.clone());
                return OrderResult {
                    order_id,
                    status: OrderStatus::Accepted,
                    trades: vec![],
                    book_snap: BookSnapshot {
                        bids_changed: vec![PriceLevel { price, qty: level.qty }],
                        asks_changed: vec![],
                    },
                };
            }
        }

        let level = PriceLevel { price, qty };
        let pos = self.bids.iter().position(|l| l.price < price);

        match pos {
            Some(idx) => {
                self.bids.insert(idx, level.clone());
            }
            None => {
                self.bids.push(level.clone());
            }
        }

        self.order_ids.push(order_id.clone());

        OrderResult {
            order_id,
            status: OrderStatus::Accepted,
            trades: vec![],
            book_snap: BookSnapshot {
                bids_changed: vec![level],
                asks_changed: vec![],
            },
        }
    }
}
