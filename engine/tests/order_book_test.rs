use engine::order_book::OrderBook;
use engine::types::{OrderStatus, PriceLevel, Side, Trade};

#[test]
fn test_empty_book_insert() {
    let mut book = OrderBook::new();
    let result = book.place_limit_buy("ORD-1".into(), 50000, 1);

    assert_eq!(result.status, OrderStatus::Accepted);
    assert_eq!(result.trades.len(), 0);
    assert_eq!(
        result.book_snap.bids_changed,
        vec![PriceLevel {
            price: 50000,
            qty: 1
        }]
    );
    assert_eq!(result.book_snap.asks_changed.len(), 0);
    assert_eq!(result.order_id, "ORD-1");
}

#[test]
fn test_empty_book_two_different_prices() {
    let mut book = OrderBook::new();
    let _result1 = book.place_limit_buy("ORD-1".into(), 50000, 1);
    let result2 = book.place_limit_buy("ORD-2".into(), 51000, 1);

    assert_eq!(result2.status, OrderStatus::Accepted);
    assert_eq!(
        result2.book_snap.bids_changed,
        vec![PriceLevel {
            price: 51000,
            qty: 1
        }]
    );
}

#[test]
fn test_same_price_aggregation() {
    let mut book = OrderBook::new();
    book.place_limit_buy("ORD-1".into(), 50000, 1);
    let result = book.place_limit_buy("ORD-2".into(), 50000, 2);

    assert_eq!(result.status, OrderStatus::Accepted);
    assert_eq!(
        result.book_snap.bids_changed,
        vec![PriceLevel {
            price: 50000,
            qty: 3
        }]
    );
}

#[test]
fn test_insert_between_levels() {
    let mut book = OrderBook::new();
    book.place_limit_buy("ORD-1".into(), 50000, 1);
    book.place_limit_buy("ORD-3".into(), 48000, 1);
    let result = book.place_limit_buy("ORD-2".into(), 49000, 1);

    assert_eq!(result.status, OrderStatus::Accepted);
    assert_eq!(
        result.book_snap.bids_changed,
        vec![PriceLevel {
            price: 49000,
            qty: 1
        }]
    );
}

#[test]
fn test_reject_price_zero() {
    let mut book = OrderBook::new();
    let result = book.place_limit_buy("ORD-1".into(), 0, 1);

    assert_eq!(result.status, OrderStatus::Rejected);
    assert_eq!(result.book_snap.bids_changed.len(), 0);
    assert_eq!(result.trades.len(), 0);
}

#[test]
fn test_reject_qty_zero() {
    let mut book = OrderBook::new();
    let result = book.place_limit_buy("ORD-1".into(), 50000, 0);

    assert_eq!(result.status, OrderStatus::Rejected);
    assert_eq!(result.book_snap.bids_changed.len(), 0);
    assert_eq!(result.trades.len(), 0);
}

#[test]
fn test_reject_duplicate_id() {
    let mut book = OrderBook::new();
    book.place_limit_buy("ORD-1".into(), 50000, 1);
    let result = book.place_limit_buy("ORD-1".into(), 51000, 1);

    assert_eq!(result.status, OrderStatus::Rejected);
    assert_eq!(result.book_snap.bids_changed.len(), 0);
}

#[test]
fn test_reject_empty_id() {
    let mut book = OrderBook::new();
    let result = book.place_limit_buy("".into(), 50000, 1);

    assert_eq!(result.status, OrderStatus::Rejected);
    assert_eq!(result.book_snap.bids_changed.len(), 0);
}

#[test]
fn test_empty_book_sell_insert() {
    let mut book = OrderBook::new();
    let result = book.place_limit_sell("ORD-1".into(), 50000, 1);

    assert_eq!(result.status, OrderStatus::Accepted);
    assert_eq!(result.trades.len(), 0);
    assert_eq!(
        result.book_snap.asks_changed,
        vec![PriceLevel {
            price: 50000,
            qty: 1
        }]
    );
    assert_eq!(result.book_snap.bids_changed.len(), 0);
    assert_eq!(result.order_id, "ORD-1");
}

#[test]
fn test_empty_book_sell_two_prices() {
    let mut book = OrderBook::new();
    let _result1 = book.place_limit_sell("ORD-1".into(), 50000, 1);
    let result2 = book.place_limit_sell("ORD-2".into(), 52000, 1);

    assert_eq!(result2.status, OrderStatus::Accepted);
    assert_eq!(
        result2.book_snap.asks_changed,
        vec![PriceLevel {
            price: 52000,
            qty: 1
        }]
    );
}

#[test]
fn test_sell_same_price_aggregation() {
    let mut book = OrderBook::new();
    book.place_limit_sell("ORD-1".into(), 50000, 1);
    let result = book.place_limit_sell("ORD-2".into(), 50000, 2);

    assert_eq!(result.status, OrderStatus::Accepted);
    assert_eq!(
        result.book_snap.asks_changed,
        vec![PriceLevel {
            price: 50000,
            qty: 3
        }]
    );
    assert_eq!(result.book_snap.bids_changed.len(), 0);
}

#[test]
fn test_sell_insert_between_levels() {
    let mut book = OrderBook::new();
    book.place_limit_sell("ORD-1".into(), 50000, 1);
    book.place_limit_sell("ORD-3".into(), 52000, 1);
    let result = book.place_limit_sell("ORD-2".into(), 51000, 1);

    assert_eq!(result.status, OrderStatus::Accepted);
    assert_eq!(
        result.book_snap.asks_changed,
        vec![PriceLevel {
            price: 51000,
            qty: 1
        }]
    );
}

#[test]
fn test_sell_reject_price_zero() {
    let mut book = OrderBook::new();
    let result = book.place_limit_sell("ORD-1".into(), 0, 1);

    assert_eq!(result.status, OrderStatus::Rejected);
    assert_eq!(result.book_snap.asks_changed.len(), 0);
    assert_eq!(result.trades.len(), 0);
}

#[test]
fn test_sell_reject_qty_zero() {
    let mut book = OrderBook::new();
    let result = book.place_limit_sell("ORD-1".into(), 50000, 0);

    assert_eq!(result.status, OrderStatus::Rejected);
    assert_eq!(result.book_snap.asks_changed.len(), 0);
    assert_eq!(result.trades.len(), 0);
}

#[test]
fn test_sell_reject_duplicate_id() {
    let mut book = OrderBook::new();
    book.place_limit_buy("ORD-1".into(), 50000, 1);
    let result = book.place_limit_sell("ORD-1".into(), 51000, 1);

    assert_eq!(result.status, OrderStatus::Rejected);
    assert_eq!(result.book_snap.asks_changed.len(), 0);
    assert_eq!(result.book_snap.bids_changed.len(), 0);
}

#[test]
fn test_partial_fill_buy_larger_than_ask() {
    let mut book = OrderBook::new();
    book.place_limit_sell("ORD-S1".into(), 50000, 2);
    let result = book.place_limit_buy("ORD-B1".into(), 51000, 5);

    assert_eq!(result.status, OrderStatus::Accepted);
    assert_eq!(
        result.trades,
        vec![Trade {
            price: 50000,
            qty: 2,
            taker_side: Side::Buy,
        }]
    );
    assert_eq!(
        result.book_snap.asks_changed,
        vec![PriceLevel {
            price: 50000,
            qty: 0,
        }]
    );
    assert_eq!(
        result.book_snap.bids_changed,
        vec![PriceLevel {
            price: 51000,
            qty: 3,
        }]
    );
}

#[test]
fn test_partial_fill_buy_smaller_than_ask() {
    let mut book = OrderBook::new();
    book.place_limit_sell("ORD-S1".into(), 50000, 5);
    let result = book.place_limit_buy("ORD-B1".into(), 51000, 2);

    assert_eq!(result.status, OrderStatus::Accepted);
    assert_eq!(
        result.trades,
        vec![Trade {
            price: 50000,
            qty: 2,
            taker_side: Side::Buy,
        }]
    );
    assert_eq!(
        result.book_snap.asks_changed,
        vec![PriceLevel {
            price: 50000,
            qty: 3,
        }]
    );
    assert_eq!(result.book_snap.bids_changed.len(), 0);
}

#[test]
fn test_partial_fill_sell_smaller_than_bid() {
    let mut book = OrderBook::new();
    book.place_limit_buy("ORD-B1".into(), 50000, 5);
    let result = book.place_limit_sell("ORD-S1".into(), 49000, 2);

    assert_eq!(result.status, OrderStatus::Accepted);
    assert_eq!(
        result.trades,
        vec![Trade {
            price: 50000,
            qty: 2,
            taker_side: Side::Sell,
        }]
    );
    assert_eq!(
        result.book_snap.bids_changed,
        vec![PriceLevel {
            price: 50000,
            qty: 3,
        }]
    );
    assert_eq!(result.book_snap.asks_changed.len(), 0);
}

#[test]
fn test_partial_fill_sell_larger_than_bid() {
    let mut book = OrderBook::new();
    book.place_limit_buy("ORD-B1".into(), 50000, 2);
    let result = book.place_limit_sell("ORD-S1".into(), 49000, 5);

    assert_eq!(result.status, OrderStatus::Accepted);
    assert_eq!(
        result.trades,
        vec![Trade {
            price: 50000,
            qty: 2,
            taker_side: Side::Sell,
        }]
    );
    assert_eq!(
        result.book_snap.bids_changed,
        vec![PriceLevel {
            price: 50000,
            qty: 0,
        }]
    );
    assert_eq!(
        result.book_snap.asks_changed,
        vec![PriceLevel {
            price: 49000,
            qty: 3,
        }]
    );
}

#[test]
fn test_sell_reject_empty_id() {
    let mut book = OrderBook::new();
    let result = book.place_limit_sell("".into(), 50000, 1);

    assert_eq!(result.status, OrderStatus::Rejected);
    assert_eq!(result.book_snap.asks_changed.len(), 0);
}

#[test]
fn test_cross_buy_single_ask_exact_match() {
    let mut book = OrderBook::new();
    book.place_limit_sell("ORD-S1".into(), 50000, 3);
    let result = book.place_limit_buy("ORD-B1".into(), 51000, 3);

    assert_eq!(result.status, OrderStatus::Accepted);
    assert_eq!(result.order_id, "ORD-B1");
    assert_eq!(
        result.trades,
        vec![Trade {
            price: 50000,
            qty: 3,
            taker_side: Side::Buy,
        }]
    );
    assert_eq!(
        result.book_snap.asks_changed,
        vec![PriceLevel {
            price: 50000,
            qty: 0,
        }]
    );
    assert_eq!(result.book_snap.bids_changed.len(), 0);
}

#[test]
fn test_no_cross_buy_below_ask() {
    let mut book = OrderBook::new();
    book.place_limit_sell("ORD-S1".into(), 50000, 2);
    let result = book.place_limit_buy("ORD-B1".into(), 49000, 2);

    assert_eq!(result.status, OrderStatus::Accepted);
    assert_eq!(result.trades.len(), 0);
    assert_eq!(result.book_snap.asks_changed.len(), 0);
    assert_eq!(
        result.book_snap.bids_changed,
        vec![PriceLevel {
            price: 49000,
            qty: 2,
        }]
    );
}

#[test]
fn test_cross_buy_ask_exact_price() {
    let mut book = OrderBook::new();
    book.place_limit_sell("ORD-S1".into(), 50000, 2);
    let result = book.place_limit_buy("ORD-B1".into(), 50000, 2);

    assert_eq!(result.status, OrderStatus::Accepted);
    assert_eq!(
        result.trades,
        vec![Trade {
            price: 50000,
            qty: 2,
            taker_side: Side::Buy,
        }]
    );
    assert_eq!(
        result.book_snap.asks_changed,
        vec![PriceLevel {
            price: 50000,
            qty: 0,
        }]
    );
    assert_eq!(result.book_snap.bids_changed.len(), 0);
}

#[test]
fn test_sweep_buy_multi_level_partial() {
    let mut book = OrderBook::new();
    book.place_limit_sell("ORD-S1".into(), 50000, 2);
    book.place_limit_sell("ORD-S2".into(), 52000, 3);
    let result = book.place_limit_buy("ORD-B1".into(), 53000, 4);

    assert_eq!(result.status, OrderStatus::Accepted);
    assert_eq!(
        result.trades,
        vec![
            Trade {
                price: 50000,
                qty: 2,
                taker_side: Side::Buy,
            },
            Trade {
                price: 52000,
                qty: 2,
                taker_side: Side::Buy,
            },
        ]
    );
    assert_eq!(
        result.book_snap.asks_changed,
        vec![
            PriceLevel {
                price: 50000,
                qty: 0,
            },
            PriceLevel {
                price: 52000,
                qty: 1,
            },
        ]
    );
    assert_eq!(result.book_snap.bids_changed.len(), 0);
}

#[test]
fn test_sweep_buy_multi_level_residual() {
    let mut book = OrderBook::new();
    book.place_limit_sell("ORD-S1".into(), 50000, 1);
    book.place_limit_sell("ORD-S2".into(), 51000, 1);
    book.place_limit_sell("ORD-S3".into(), 52000, 1);
    let result = book.place_limit_buy("ORD-B1".into(), 53000, 5);

    assert_eq!(result.status, OrderStatus::Accepted);
    assert_eq!(
        result.trades,
        vec![
            Trade {
                price: 50000,
                qty: 1,
                taker_side: Side::Buy,
            },
            Trade {
                price: 51000,
                qty: 1,
                taker_side: Side::Buy,
            },
            Trade {
                price: 52000,
                qty: 1,
                taker_side: Side::Buy,
            },
        ]
    );
    assert_eq!(
        result.book_snap.asks_changed,
        vec![
            PriceLevel {
                price: 50000,
                qty: 0,
            },
            PriceLevel {
                price: 51000,
                qty: 0,
            },
            PriceLevel {
                price: 52000,
                qty: 0,
            },
        ]
    );
    assert_eq!(
        result.book_snap.bids_changed,
        vec![PriceLevel {
            price: 53000,
            qty: 2,
        }]
    );
}

#[test]
fn test_sweep_sell_multi_level_residual() {
    let mut book = OrderBook::new();
    book.place_limit_buy("ORD-B1".into(), 51000, 2);
    book.place_limit_buy("ORD-B2".into(), 50000, 3);
    let result = book.place_limit_sell("ORD-S1".into(), 49000, 6);

    assert_eq!(result.status, OrderStatus::Accepted);
    assert_eq!(
        result.trades,
        vec![
            Trade {
                price: 51000,
                qty: 2,
                taker_side: Side::Sell,
            },
            Trade {
                price: 50000,
                qty: 3,
                taker_side: Side::Sell,
            },
        ]
    );
    assert_eq!(
        result.book_snap.bids_changed,
        vec![
            PriceLevel {
                price: 51000,
                qty: 0,
            },
            PriceLevel {
                price: 50000,
                qty: 0,
            },
        ]
    );
    assert_eq!(
        result.book_snap.asks_changed,
        vec![PriceLevel {
            price: 49000,
            qty: 1,
        }]
    );
}
