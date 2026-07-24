# Research: Order Matching (Crossing Engine)

## Decision: Extend existing methods rather than create separate matching engine

**Rationale**: `place_limit_buy` and `place_limit_sell` already own the order insertion flow. Adding crossing logic inline (validation → crossing loop → residual placement) keeps the state machine in one place. No new module boundary, no synchronization between "inserter" and "matcher" — crossing happens atomically within a single method call.

**Alternatives considered**:
- Separate `match_order()` function: Splits validation from execution, risks calling matching without prior validation. Rejected.
- Trait-based matching strategy: Heap allocation for dynamic dispatch. Rejected (Constitution II).

## Decision: Pre-allocated trade buffer in OrderBook

**Rationale**: `OrderBook` gains `trades_buf: Vec<Trade>` initialized with `Vec::with_capacity(128)`. Each operation: `clear()` (no deallocation, preserves capacity), `push()` during crossing (no reallocation while within capacity), then `clone()` for OrderResult. Single allocation per order for the clone; inner crossing loop is zero-alloc. Buffer capacity (128) exceeds any realistic multi-level sweep for a single trading pair.

**Alternatives considered**:
- `Vec::with_capacity()` per operation without buffer: Allocates new Vec each call. Rejected (aim for minimal allocations).
- Fixed-size array `[Trade; 128]` with index counter: Requires `Copy` trait on Trade, forces all stack allocation. Viable but adds complexity for tracking count. Buffer Vec is simpler, still zero-alloc in loop.
- `std::mem::take` + re-allocate capacity: Still one allocation per call to restore buffer, no advantage over clone.

## Decision: Linear scan crossing from best price (index 0)

**Rationale**: Bids are sorted descending (best = highest price at index 0). Asks are sorted ascending (best = lowest price at index 0). The crossing loop iterates from index 0:
- **Buy crossing asks**: while `asks[0].price <= incoming_price && remaining_qty > 0`, cross.
- **Sell crossing bids**: while `bids[0].price >= incoming_price && remaining_qty > 0`, cross.

When a level is fully consumed (`level.qty <= remaining_qty`), remove it via `Vec::remove(0)` — shifts remaining elements left, next iteration checks new index 0 (the next best price). When a level is partially consumed, mutate in-place and exit loop.

O(n) linear scan, same cache efficiency as 001/002. No binary search needed — we always start from index 0 (best price).

**Alternatives considered**:
- Binary search for first crossing level then drain forward: Binary search still O(log n + k) for k levels consumed; same overall O(n) due to Vec::remove shifts. More complex, no benefit. Rejected.
- Reverse iteration with swap_remove: Sacrifices ordering for O(1) removal. Rejected — order must be preserved for subsequent operations.

## Decision: Trade struct expansion — price, qty, taker_side

**Rationale**: Replace empty `Trade {}` with:
```rust
pub struct Trade {
    pub price: u64,
    pub qty: u64,
    pub taker_side: Side,
}
```
Three fields, all `Copy`, 24 bytes total. Clone is memcpy. No heap allocation. `taker_side` enables gateway to distinguish buy-initiated vs sell-initiated trades for Observable Operations (Constitution).

**Alternatives considered**:
- Include maker_side: Redundant — inferred from taker_side. Adds 1 byte, no benefit. Rejected.
- Include timestamp/trade_id: Requires `Instant` or `String` — heap allocation. Out of scope. Rejected.
- Keep Trade empty, encode in separate result field: Breaks existing OrderResult contract. Rejected.

## Decision: BookSnapshot reports removed levels with qty=0

**Rationale**: Per clarification Q1 — fully consumed levels appear in `BookSnapshot` with `qty=0`. This enables consumers to distinguish "level was removed" from "level was not touched". SC-005 (no zero-qty in order book state) governs the book itself, not the change snapshot.

Implementation: before `Vec::remove(0)`, push `PriceLevel { price, qty: 0 }` to the appropriate `*_changed` vector. Then remove. Consistency with partial reductions (which push the new qty).

**Alternatives considered**:
- Omit removed levels from snapshot: Cannot verify removal in tests. Root cause of Q1. Rejected.
- Push pre-removal qty: Consumer must compute delta. More work for no gain. Rejected.

## Decision: Clone-based result construction (acceptable allocation)

**Rationale**: `OrderResult` receives `self.trades_buf.clone()` for the trades field, and `Vec::new()` or single-element `vec![]` for BookSnapshot entries. These allocations happen once per operation call, not per trade. The inner crossing loop performs zero heap allocation — only in-place mutations and `Vec::remove(0)` shifts. This satisfies Constitution II's spirit: the hot loop is allocation-free.

Total allocations per `place_limit_buy`/`place_limit_sell`:
1. Trade Vec clone (N trades, where N ≤ levels consumed)
2. BookSnapshot Vec allocations (existing pattern from 001/002)
3. Potential reallocation if snapshot has >1 changed level

This is identical to the existing allocation profile of 001/002 (which allocate for `bids_changed`/`asks_changed` Vecs on every call).

**Alternatives considered**:
- Pre-allocate snapshot Vecs: Premature optimization. Snapshot is typically 1-3 levels. Rejected.
- Return references: Lifetime nightmare, incompatible with current API. Rejected.
