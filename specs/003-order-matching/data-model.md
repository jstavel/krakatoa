# Data Model: Order Matching (Crossing Engine)

## Changes from 002-limit-sell

This feature extends the existing OrderBook with crossing logic. Types `Side`, `OrderStatus`, `PriceLevel`, `BookSnapshot`, and `OrderResult` are retained from 001/002. `Trade` struct gains real fields.

## OrderBook (extended)

| Field | Type | Description |
|-------|------|-------------|
| bids | Vec\<PriceLevel\> | Unchanged — sorted descending by price (best bid at index 0) |
| asks | Vec\<PriceLevel\> | Unchanged — sorted ascending by price (best ask at index 0) |
| order_ids | Vec\<String\> | Unchanged — shared cross-side uniqueness, append-only |
| trades_buf | Vec\<Trade\> | **NEW** — pre-allocated with `with_capacity(128)`, cleared per operation |

## Trade (modified)

| Field | Type | Description |
|-------|------|-------------|
| price | u64 | Execution price (resting/maker order price) |
| qty | u64 | Filled quantity for this trade |
| taker_side | Side | Side of the incoming order (Buy or Sell) |

Previously `Trade {}` (empty struct). Now 3 fields, total 24 bytes, `Clone`/`Copy`.

## State Transitions

### place_limit_buy(order_id, price, qty)

```
validate:
  price=0 || qty=0 || order_id empty || order_id duplicate → Rejected (unchanged)

crossing loop (asks non-empty, price >= asks[0].price, remaining_qty > 0):
  for each ask level from index 0:
    if ask.qty <= remaining_qty:
      push Trade{price: ask.price, qty: ask.qty, taker_side: Buy} → trades_buf
      push PriceLevel{ask.price, qty: 0} → asks_changed
      remaining_qty -= ask.qty
      remove asks[0]
      continue
    else (ask.qty > remaining_qty):
      push Trade{price: ask.price, qty: remaining_qty, taker_side: Buy} → trades_buf
      ask.qty -= remaining_qty
      push PriceLevel{ask.price, qty: ask.qty} → asks_changed
      remaining_qty = 0
      break

post-crossing:
  if remaining_qty > 0:
    insert remaining_qty at price on bids side (existing logic)
    push inserted level → bids_changed
  order_ids.push(order_id)
  return Accepted with trades_buf.clone(), book_snap
```

### place_limit_sell(order_id, price, qty)

```
validate: (same as buy, unchanged)

crossing loop (bids non-empty, price <= bids[0].price, remaining_qty > 0):
  for each bid level from index 0:
    if bid.qty <= remaining_qty:
      push Trade{price: bid.price, qty: bid.qty, taker_side: Sell} → trades_buf
      push PriceLevel{bid.price, qty: 0} → bids_changed
      remaining_qty -= bid.qty
      remove bids[0]
      continue
    else (bid.qty > remaining_qty):
      push Trade{price: bid.price, qty: remaining_qty, taker_side: Sell} → trades_buf
      bid.qty -= remaining_qty
      push PriceLevel{bid.price, qty: bid.qty} → bids_changed
      remaining_qty = 0
      break

post-crossing:
  if remaining_qty > 0:
    insert remaining_qty at price on asks side (existing logic)
    push inserted level → asks_changed
  order_ids.push(order_id)
  return Accepted with trades_buf.clone(), book_snap
```

## Crossing Conditions

| Incoming | Opposing Side | Crossing Criterion | Trade Price |
|----------|---------------|--------------------|-------------|
| Buy | Asks (best = lowest, index 0) | `price >= asks[0].price` | `asks[i].price` (maker) |
| Sell | Bids (best = highest, index 0) | `price <= bids[0].price` | `bids[i].price` (maker) |

## Validation Rules (unchanged from 001/002)

| Rule                                         | Source            |
|----------------------------------------------|-------------------|
| order_id must be non-empty                   | FR-012 / existing |
| order_id must be unique across bids AND asks | FR-012 / existing |
| price must be > 0                            | FR-012 / existing |
| quantity must be > 0                         | FR-012 / existing |

Validation occurs BEFORE any crossing logic. If rejected, no state mutation, empty BookSnapshot, empty trades.

## Invariants

1. Bids sorted descending by price (highest at index 0) — FR-010
2. Asks sorted ascending by price (lowest at index 0) — FR-010
3. Each price appears at most once in bids (aggregated)
4. Each price appears at most once in asks (aggregated)
5. No order_id exists in both bids and asks simultaneously
6. Rejected orders produce empty BookSnapshot and empty trades
7. Zero-quantity levels never exist in the book after operation completion — SC-005
8. Fully consumed levels appear in BookSnapshot with qty=0 — FR-006
9. Trades are ordered by execution sequence (ascending ask for buys, descending bid for sells) — FR-011
10. Fully filled orders have no residual — FR-008
11. Partially filled orders have residual at incoming price — FR-009
