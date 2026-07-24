# Quickstart: Order Matching (Crossing Engine)

## Prerequisites

- Rust 1.75+ (stable)
- `cargo` installed
- Repository cloned: `git clone <repo> && cd krakatoa/engine`

## Verify Baseline

```bash
cd engine
cargo test
```

Expected: 16 tests pass (8 buy + 8 sell from features 001/002). This confirms the baseline before crossing changes.

## Run Full Test Suite (after implementation)

```bash
cd engine
cargo test
```

Expected: 28+ tests pass (16 regression + ~12 new crossing tests).

## Validation Scenarios

### Scenario 1: Buy crosses single ask (US1)

1. Pre-populate book with one ask at 50000 qty 3
2. Submit `place_limit_buy("ORD-B", 51000, 3)`
3. Verify: status=Accepted, 1 trade at 50000 qty 3, asks empty, no residual bid

### Scenario 2: Buy does NOT cross (price below best ask)

1. Pre-populate book with one ask at 50000 qty 2
2. Submit `place_limit_buy("ORD-B", 49000, 2)`
3. Verify: status=Accepted, 0 trades, buy placed on bids at 49000, ask unchanged

### Scenario 3: Partial fill — buy larger than ask (US2)

1. Pre-populate book with one ask at 50000 qty 2
2. Submit `place_limit_buy("ORD-B", 51000, 5)`
3. Verify: status=Accepted, 1 trade at 50000 qty 2, ask consumed (qty=0 in snapshot), residual buy at 50000 qty 3 on bids

### Scenario 4: Partial fill — buy smaller than ask (US2)

1. Pre-populate book with one ask at 50000 qty 5
2. Submit `place_limit_buy("ORD-B", 51000, 2)`
3. Verify: status=Accepted, 1 trade at 50000 qty 2, ask reduced to qty 3, no residual

### Scenario 5: Multi-level sweep (US3)

1. Pre-populate book with asks at [50000 qty 2, 52000 qty 3]
2. Submit `place_limit_buy("ORD-B", 53000, 4)`
3. Verify: status=Accepted, 2 trades [(50000,2), (52000,2)], 50000 ask consumed (qty=0), 52000 ask remains qty=1, no residual

### Scenario 6: Multi-level sweep with residual (US3)

1. Pre-populate book with asks at [50000 qty 1, 51000 qty 1, 52000 qty 1]
2. Submit `place_limit_buy("ORD-B", 53000, 5)`
3. Verify: status=Accepted, 3 trades, all asks consumed, residual buy at 53000 qty 2 on bids

### Scenario 7: Sell crosses bid (symmetric validation)

1. Pre-populate book with one bid at 50000 qty 5
2. Submit `place_limit_sell("ORD-S", 49000, 2)`
3. Verify: status=Accepted, 1 trade at 50000 qty 2 (maker=bid), bid reduced to qty 3, no residual

### Scenario 8: Regression — no crossing when opposing side empty

1. Empty book
2. Submit `place_limit_buy("ORD-B", 50000, 1)`
3. Verify: status=Accepted, 0 trades, buy placed on bids (unchanged behavior from 001)

## Key Observability Points

- Each Trade record contains: `price` (maker price), `qty` (filled amount), `taker_side` (Buy or Sell)
- BookSnapshot reports removed levels with `qty=0` — check `bids_changed`/`asks_changed` for zero-quantity entries
- Trades in OrderResult are ordered by execution sequence
- Rejected orders (validation failure) have empty `trades` and empty `BookSnapshot`

## Test Organization

All tests in `engine/tests/order_book_test.rs`. New tests follow naming convention:
- `test_cross_*` — crossing scenarios
- `test_partial_fill_*` — partial fill scenarios
- `test_sweep_*` — multi-level sweep scenarios
- `test_no_cross_*` — non-crossing regression scenarios
