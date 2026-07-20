# Quickstart: Limit Buy Order Validation

Prove the feature works end-to-end without external dependencies.

## Prerequisites

- Rust toolchain (1.75+): `rustc --version`
- Krakatoa engine at `engine/`

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

All unit tests pass. Test coverage per user story:

| User Story | Tests |
|---|---|
| US1 (P1) | Empty book insert → Accepted. Two different prices → sorted descending. |
| US2 (P2) | Same-price aggregation → quantity sum. Insert between levels → correct sort. |
| US3 (P3) | price=0 → Rejected. qty=0 → Rejected. Duplicate ID → Rejected. |

## Manual Validation (REPL-style)

```bash
cargo test -- --nocapture order_book_test
```

Verify each test prints its scenario and passes.

## Files Involved

- `engine/src/types.rs` — Side, OrderStatus, PriceLevel, Trade, BookSnapshot, OrderResult
- `engine/src/order_book.rs` — OrderBook::new(), place_limit_buy()
- `engine/tests/order_book_test.rs` — All acceptance scenario tests
