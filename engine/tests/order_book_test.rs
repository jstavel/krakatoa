use engine::order_book::OrderBook;
use engine::types::{OrderStatus, PriceLevel};

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
