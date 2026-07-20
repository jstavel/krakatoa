# Research: Limit Sell Order

## Decision: Extend existing OrderBook rather than create separate AskBook

**Rationale**: The order book is one structure — bids and asks are two sides of
the same data structure. Shared `order_ids` set enables cross-side uniqueness
(FR-007) naturally. No new module boundary needed.

**Alternatives considered**:
- Separate `AskBook` struct: code duplication, harder to enforce cross-side ID uniqueness. Rejected.

## Decision: Ascending sort order via reversed comparison

**Rationale**: Bids use `position(|l| l.price < price)` — find first level
strictly lower than new price (insert before it for descending). Asks use
`position(|l| l.price > price)` — find first level strictly higher than new price
(insert before it for ascending). Same O(n) linear scan, same cache efficiency.

**Alternatives considered**:
- Binary search with custom comparator: Overkill for MVP, same insert O(n). Rejected.
- Prepend + sort for asks: O(n log n). Rejected.

## Decision: Shared order_ids Vec for cross-side uniqueness

**Rationale**: Already have `order_ids: Vec<String>` from 001. `place_limit_sell`
uses the same `self.order_ids.contains(&order_id)` check — no additional structure
needed. O(n) scan, n is small, no heap allocation.

**Alternatives considered**:
- Separate per-side ID tracking: Cannot enforce FR-007 (cross-side uniqueness). Rejected.
- HashMap for O(1) lookup: heap allocation. Rejected for MVP (memory pool planned).

## Decision: Same clone() strategy as 001

**Rationale**: PriceLevel is 16 bytes. clone() = memcpy, 1-2 CPU cycles.
No heap allocation. Same pattern as place_limit_buy.
