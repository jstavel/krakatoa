use crate::types::{BookSnapshot, OrderResult, OrderStatus, PriceLevel, Side, Trade};

pub struct OrderBook {
    bids: Vec<PriceLevel>,
    asks: Vec<PriceLevel>,
    order_ids: Vec<String>,
    trades_buf: Vec<Trade>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            bids: Vec::with_capacity(128),
            asks: Vec::with_capacity(128),
            order_ids: Vec::with_capacity(128),
            trades_buf: Vec::with_capacity(128),
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

        self.trades_buf.clear();
        let mut remaining_qty = qty;
        let bids_changed: Vec<PriceLevel> = Vec::new();
        let mut asks_changed: Vec<PriceLevel> = Vec::new();

        while !self.asks.is_empty() && price >= self.asks[0].price && remaining_qty > 0 {
            let ask_qty = self.asks[0].qty;
            if ask_qty <= remaining_qty {
                self.trades_buf.push(Trade {
                    price: self.asks[0].price,
                    qty: ask_qty,
                    taker_side: Side::Buy,
                });
                asks_changed.push(PriceLevel {
                    price: self.asks[0].price,
                    qty: 0,
                });
                remaining_qty -= ask_qty;
                self.asks.remove(0);
            } else {
                self.trades_buf.push(Trade {
                    price: self.asks[0].price,
                    qty: remaining_qty,
                    taker_side: Side::Buy,
                });
                self.asks[0].qty -= remaining_qty;
                asks_changed.push(PriceLevel {
                    price: self.asks[0].price,
                    qty: self.asks[0].qty,
                });
                remaining_qty = 0;
            }
        }

        if remaining_qty > 0 {
            for level in self.bids.iter_mut() {
                if level.price == price {
                    level.qty += remaining_qty;
                    self.order_ids.push(order_id.clone());
                    return OrderResult {
                        order_id,
                        status: OrderStatus::Accepted,
                        trades: self.trades_buf.clone(),
                        book_snap: BookSnapshot {
                            bids_changed: vec![PriceLevel {
                                price,
                                qty: level.qty,
                            }],
                            asks_changed,
                        },
                    };
                }
            }

            let level = PriceLevel {
                price,
                qty: remaining_qty,
            };
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

            return OrderResult {
                order_id,
                status: OrderStatus::Accepted,
                trades: self.trades_buf.clone(),
                book_snap: BookSnapshot {
                    bids_changed: vec![level],
                    asks_changed,
                },
            };
        }

        self.order_ids.push(order_id.clone());

        OrderResult {
            order_id,
            status: OrderStatus::Accepted,
            trades: self.trades_buf.clone(),
            book_snap: BookSnapshot {
                bids_changed,
                asks_changed,
            },
        }
    }

    pub fn place_limit_sell(&mut self, order_id: String, price: u64, qty: u64) -> OrderResult {
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

        self.trades_buf.clear();
        let mut remaining_qty = qty;
        let mut bids_changed: Vec<PriceLevel> = Vec::new();
        let asks_changed: Vec<PriceLevel> = Vec::new();

        while !self.bids.is_empty() && price <= self.bids[0].price && remaining_qty > 0 {
            let bid_qty = self.bids[0].qty;
            if bid_qty <= remaining_qty {
                self.trades_buf.push(Trade {
                    price: self.bids[0].price,
                    qty: bid_qty,
                    taker_side: Side::Sell,
                });
                bids_changed.push(PriceLevel {
                    price: self.bids[0].price,
                    qty: 0,
                });
                remaining_qty -= bid_qty;
                self.bids.remove(0);
            } else {
                self.trades_buf.push(Trade {
                    price: self.bids[0].price,
                    qty: remaining_qty,
                    taker_side: Side::Sell,
                });
                self.bids[0].qty -= remaining_qty;
                bids_changed.push(PriceLevel {
                    price: self.bids[0].price,
                    qty: self.bids[0].qty,
                });
                remaining_qty = 0;
            }
        }

        if remaining_qty > 0 {
            for level in self.asks.iter_mut() {
                if level.price == price {
                    level.qty += remaining_qty;
                    self.order_ids.push(order_id.clone());
                    return OrderResult {
                        order_id,
                        status: OrderStatus::Accepted,
                        trades: self.trades_buf.clone(),
                        book_snap: BookSnapshot {
                            bids_changed,
                            asks_changed: vec![PriceLevel {
                                price,
                                qty: level.qty,
                            }],
                        },
                    };
                }
            }

            let level = PriceLevel {
                price,
                qty: remaining_qty,
            };
            let pos = self.asks.iter().position(|l| l.price > price);

            match pos {
                Some(idx) => {
                    self.asks.insert(idx, level.clone());
                }
                None => {
                    self.asks.push(level.clone());
                }
            }

            self.order_ids.push(order_id.clone());

            return OrderResult {
                order_id,
                status: OrderStatus::Accepted,
                trades: self.trades_buf.clone(),
                book_snap: BookSnapshot {
                    bids_changed,
                    asks_changed: vec![level],
                },
            };
        }

        self.order_ids.push(order_id.clone());

        OrderResult {
            order_id,
            status: OrderStatus::Accepted,
            trades: self.trades_buf.clone(),
            book_snap: BookSnapshot {
                bids_changed,
                asks_changed,
            },
        }
    }
}
