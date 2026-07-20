# Data Model: Limit Sell Order

## Changes from 001-limit-buy

This feature extends the existing OrderBook. All types (Side, OrderStatus,
PriceLevel, Trade, BookSnapshot, OrderResult) are unchanged from 001.

## OrderBook (extended)

| Field | Type | Description |
|-------|------|-------------|
| bids | Vec\<PriceLevel\> | Unchanged — sorted descending by price |
| asks | Vec\<PriceLevel\> | **NEW** — sorted ascending by price (lowest first) |
| order_ids | Vec\<String\> | Unchanged — shared cross-side uniqueness |

## New Operation: place_limit_sell

```text
INVOKE:  place_limit_sell(order_id, price, qty)
COMPLETE: OrderResult { order_id, status, trades, book_snap }
```

## State Transitions (asks side)

```
place_limit_sell(order):
  valid + unique ID (cross-side check):
    price exists in asks → aggregate qty → Accepted
    price new → insert at ascending position → Accepted
    book_snap: asks_changed = [changed level], bids_changed = []

  invalid (price=0 || qty=0 || duplicate ID || empty ID):
    no state change → Rejected
    book_snap: bids_changed = [], asks_changed = []
```

## Validation Rules (same as 001, plus cross-side)

| Rule | Source |
|------|--------|
| order_id must be non-empty | FR-001 |
| order_id must be unique across bids AND asks | FR-007 |
| price must be > 0 | FR-005 |
| quantity must be > 0 | FR-006 |
| No matching against bids | FR-008 |

## Invariants

1. Asks sorted ascending by price (best ask = lowest price at index 0)
2. Each price appears at most once in asks (aggregated)
3. Each price appears at most once in bids (aggregated)
4. No order_id exists in both bids and asks simultaneously
5. Rejected orders produce empty BookSnapshot
6. Accepted orders echo order_id in OrderResult
