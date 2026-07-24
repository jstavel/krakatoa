# Contract: Order Book Operations (Extended — Crossing)

Per Constitution 1.2.0 (Contract First) and Observable Operations.

## place_limit_buy (extended)

```clojure
;; INVOKE:  place_limit_buy(order_id, price, qty)
;; COMPLETE: OrderResult { order_id, status, trades, book_snap }

;; NEW: crossing against asks when price >= best_ask
;; bids side, descending sort
;; RESIDUAL: unfilled qty placed on bids at incoming price
```

## place_limit_sell (extended)

```clojure
;; INVOKE:  place_limit_sell(order_id, price, qty)
;; COMPLETE: OrderResult { order_id, status, trades, book_snap }

;; NEW: crossing against bids when price <= best_bid
;; asks side, ascending sort
;; RESIDUAL: unfilled qty placed on asks at incoming price
```

## Type Schemas (Malli)

```clojure
(require '[malli.core :as m])

(def Side
  [:enum :buy :sell])

(def OrderStatus
  [:enum :accepted :rejected])

(def PriceLevel
  [:map
   [:price pos-int?]
   [:qty   nat-int?]])   ;; qty=0 allowed in BookSnapshot for removed levels

(def Trade
  [:map
   [:price      pos-int?]
   [:qty        pos-int?]
   [:taker-side Side]])

(def BookSnapshot
  [:map
   [:bids_changed [:vector PriceLevel]]
   [:asks_changed [:vector PriceLevel]]])

(def OrderResult
  [:map
   [:order-id  :string]
   [:status    OrderStatus]
   [:trades    [:vector Trade]]     ;; populated when crossing occurs
   [:book-snap BookSnapshot]])
```

## Semantics

### place_limit_buy (with crossing)

```
BEFORE crossing: validation (price > 0, qty > 0, order_id non-empty, order_id unique)

IF asks is empty OR price < best_ask:
  → place as resting bid (existing behavior, no trade generated)
  → bids_changed = [new/changed level], trades = []

IF asks non-empty AND price >= best_ask:
  → CROSSING LOOP:
    for each ask from best (lowest) upward:
      fill_qty = min(remaining_qty, ask.qty)
      push Trade{price: ask.price, qty: fill_qty, taker_side: :buy} → trades
      if ask.qty <= remaining_qty:
        push PriceLevel{ask.price, 0} → asks_changed
        remove ask
      else:
        ask.qty -= remaining_qty
        push PriceLevel{ask.price, ask.qty} → asks_changed
        remaining_qty = 0; break

  → POST-CROSSING:
    if remaining_qty > 0:
      insert at incoming price on bids
      push inserted level → bids_changed
    → Accepted, trades = [...], book_snap has bids_changed + asks_changed
```

### place_limit_sell (with crossing)

Symmetric to buy. Crosses against bids from best (highest) downward at bid prices.

```
IF bids is empty OR price > best_bid:
  → place as resting ask (existing behavior, no trade generated)

IF bids non-empty AND price <= best_bid:
  → CROSSING LOOP (symmetric, trades at bid prices with taker_side: :sell)
  → RESIDUAL (if any) placed on asks at incoming price
```

### Observable Operations

- `order-id` echoed in OrderResult for invoke↔complete pairing
- Each Trade carries `taker-side` — unambiguous which side initiated
- Trades ordered chronologically (ascending ask prices for buy crossing, descending bid prices for sell crossing)
- `bids_changed` reports all affected bid levels (added, modified with new qty, removed with qty=0)
- `asks_changed` reports all affected ask levels (added, modified with new qty, removed with qty=0)

### Rejection semantics (unchanged)

- On `:rejected`: trades = [], bids_changed = [], asks_changed = []
- Validation unchanged: price=0, qty=0, empty order_id, duplicate order_id all trigger rejection before crossing
