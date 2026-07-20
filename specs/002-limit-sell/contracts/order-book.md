# Contract: Order Book Operations (Extended)

Per Constitution 1.2.0 (Contract First) and Observable Operations.

## place_limit_buy (unchanged from 001)

```clojure
;; INVOKE:  place_limit_buy(order_id, price, qty)
;; COMPLETE: OrderResult { order_id, status, trades, book_snap }
;; bids side, descending sort
```

## place_limit_sell (NEW)

```clojure
;; INVOKE:  place_limit_sell(order_id, price, qty)
;; COMPLETE: OrderResult { order_id, status, trades, book_snap }

(require '[malli.core :as m])

(def Side
  [:enum :buy :sell])

(def OrderStatus
  [:enum :accepted :rejected])

(def PriceLevel
  [:map
   [:price pos-int?]
   [:qty   pos-int?]])

(def Trade
  [:map])

(def BookSnapshot
  [:map
   [:bids_changed [:vector PriceLevel]]   ;; empty for sell operations
   [:asks_changed [:vector PriceLevel]]]) ;; populated for sell operations

(def OrderResult
  [:map
   [:order-id  :string]
   [:status    OrderStatus]
   [:trades    [:vector Trade]]
   [:book-snap BookSnapshot]])

(def OrderBook
  [:map
   [:bids [:vector PriceLevel]]   ;; descending
   [:asks [:vector PriceLevel]]]) ;; ascending
```

## Semantics

- `order-id` echoed for invoke↔complete pairing (Observable Operations)
- `place_limit_sell`: on `:accepted`, `asks_changed` contains affected level,
  `bids_changed` is empty
- `place_limit_sell`: on `:rejected`, both `bids_changed` and `asks_changed`
  are empty
- `place_limit_buy`: on `:accepted`, `bids_changed` contains affected level,
  `asks_changed` is empty
- Order IDs unique across both sides (FR-007)
