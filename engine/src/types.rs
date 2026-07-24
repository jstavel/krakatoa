#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrderStatus {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PriceLevel {
    pub price: u64,
    pub qty: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Trade {
    pub price: u64,
    pub qty: u64,
    pub taker_side: Side,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BookSnapshot {
    pub bids_changed: Vec<PriceLevel>,
    pub asks_changed: Vec<PriceLevel>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderResult {
    pub order_id: String,
    pub status: OrderStatus,
    pub trades: Vec<Trade>,
    pub book_snap: BookSnapshot,
}
