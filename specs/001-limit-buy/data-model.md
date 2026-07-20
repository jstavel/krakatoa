# Data Model: Limit Buy Order

## Entities

### Order

| Field    | Type   | Constraints                   | Description                                                              |
|----------|--------|-------------------------------|--------------------------------------------------------------------------|
| order_id | String | Non-empty, unique within book | Unique operation identifier (echoed in result per Observable Operations) |
| price    | u64    | > 0 (rejected if 0)           | Limit price in whole units                                               |
| quantity | u64    | > 0 (rejected if 0)           | Order size in whole units                                                |
| side     | Side   | Enum { Buy, Sell }            | Only Buy is used in this feature; Sell defined for future                |

### Side (enum)

```
Side::Buy    — this feature
Side::Sell   — future features
```

### OrderStatus (enum)

```
OrderStatus::Accepted   — order placed in book
OrderStatus::Rejected   — validation failed, no state change
```

### PriceLevel

| Field    | Type | Description                             |
|----------|------|-----------------------------------------|
| price    | u64  | Price of this level                     |
| quantity | u64  | Total quantity aggregated at this price |

### Trade

Placeholder struct, always empty in this feature. Defined now for complete data model.

### BookSnapshot

| Field        | Type              | Description                                                |
|--------------|-------------------|------------------------------------------------------------|
| bids_changed | Vec\<PriceLevel\> | Price levels that changed on bid side. Empty on rejection. |
| asks_changed | Vec\<PriceLevel\> | Always empty in this feature.                              |

### OrderResult

| Field     | Type         | Description                                          |
|-----------|--------------|------------------------------------------------------|
| order_id  | String       | Echo of submitted order_id (invoke↔complete pairing) |
| status    | OrderStatus  | Accepted or Rejected                                 |
| trades    | Vec\<Trade\> | Always empty in this feature                         |
| book_snap | BookSnapshot | Changed price levels                                 |

### OrderBook

| Field | Type              | Description                          |
|-------|-------------------|--------------------------------------|
| bids  | Vec\<PriceLevel\> | Bid side, sorted descending by price |

### State Transitions

```
OrderBook state:
  new() → empty bids

place_limit_buy(order):
  valid + unique ID:
    price exists in bids → aggregate qty → Accepted
    price new → insert at correct position → Accepted
    book_snap: bids_changed = [changed level]

  invalid (price=0 || qty=0 || duplicate ID):
    no state change → Rejected
    book_snap: bids_changed = []

  bids always maintained in descending order by price
```

## Validation Rules

| Rule | Source |
|------|--------|
| order_id must be non-empty and unique | FR-001, FR-007 |
| price must be > 0 | FR-005 |
| quantity must be > 0 | FR-006 |
| No matching against asks | FR-008 |

## Invariants

1. Bids are always sorted descending by price (best bid = highest price at index 0)
2. Each price appears at most once in bids (aggregated)
3. Rejected orders produce empty BookSnapshot
4. Accepted orders produce BookSnapshot with exactly the changed level
5. Accepted orders echo order_id in OrderResult
