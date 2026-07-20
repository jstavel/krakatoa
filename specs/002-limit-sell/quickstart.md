# Quickstart: Limit Sell Order Validation

## Prerequisites

- Rust toolchain (1.75+)
- 001-limit-buy implemented (OrderBook, bids, place_limit_buy exist)
- Existing 8 tests pass

## Setup

```bash
cd engine
cargo build
```

## Run Tests

```bash
cd engine
cargo test
```

## Expected Outcomes

All existing tests (8 from 001) plus new sell-side tests pass:

| User Story | Tests |
|---|---|
| US1 (P1) | Empty book limit sell → Accepted, asks snapshot correct |
| US2 (P2) | Same-price aggregation on asks, ascending sort between levels |
| US3 (P3) | price=0 → Rejected, qty=0 → Rejected, duplicate ID (cross-side) → Rejected, empty ID → Rejected |

## Files Modified

- `engine/src/order_book.rs` — Extended: asks field, place_limit_sell()
- `engine/tests/order_book_test.rs` — Extended: 6 new sell-side tests
