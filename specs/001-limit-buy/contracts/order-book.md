# Contract: Order Book Operations

Per Constitution 1.2.0 (Contract First) and Observable Operations.
All contracts expressed as Malli schemas.

## place_limit_buy

```clojure
;; INVOKE:  place_limit_buy(order_id, price, qty)
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
  [:map])  ;; placeholder — always empty in this feature

(def BookSnapshot
  [:map
   [:bids_changed [:vector PriceLevel]]
   [:asks_changed [:vector PriceLevel]]])

(def OrderResult
  [:map
   [:order-id  :string]        ;; echo — invoke↔complete pairing
   [:status    OrderStatus]
   [:trades    [:vector Trade]]
   [:book-snap BookSnapshot]])

(def OrderBook
  [:map
   [:bids [:vector PriceLevel]]])
```

## Semantics

- `order-id` in result echoes `order_id` from invoke — enables Jepsen linearizability verification (Constitution: Observable Operations)
- On `:accepted`: `bids_changed` contains exactly one `PriceLevel` (the affected level)
- On `:rejected`: `bids_changed` is empty vector `[]`
- `asks_changed` is always `[]` in this feature
- `trades` is always `[]` in this feature
