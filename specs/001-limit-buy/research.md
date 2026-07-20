# Research: Limit Buy Order

## Decision: Fixed-capacity Vec with pre-allocation

**Rationale**: Constitution II requires zero runtime allocation. `Vec::with_capacity(N)` allocates once at initialization, then `push` within capacity is O(1) without reallocation. After initialization, no further heap allocation occurs.

**Alternatives considered**:
- Array with fixed size: O(N) insertion sort, rejects orders beyond capacity. Rejected — order book size unknown at compile time.
- Linked list: pointer chasing ruins cache locality. Rejected — latency targets require cache-friendly data structures.
- BTreeMap (std): heap-allocates per node. Rejected — violates zero-allocation on hot path.

## Decision: Descending sort order maintained by insertion position

**Rationale**: When inserting a new bid, scan from highest price to find insertion point. Simple linear scan on small-to-medium book sizes is faster than binary search + shift due to cache locality of contiguous Vec. `Vec::insert` shifts elements right — single memmove, no allocation.

**Alternatives considered**:
- Binary search + insert: O(log N) search, O(N) insert. Worse constant factor for small N (< 100 levels).
- Always append + sort: O(N log N) per insert. Rejected — too slow.
- Sorted `Vec` with binary heap: complex, overkill for this feature.

## Decision: Price aggregation by linear scan of existing levels

**Rationale**: When inserting an order, scan existing price levels. If price matches, add quantities. If not, insert at correct position. Single pass, no allocation.

**Alternatives considered**:
- HashMap<Price, Quantity>: heap-allocates per entry. Rejected — violates Constitution II.
- Separate "insert" and "aggregate" passes: two passes unnecessary. Single pass handles both.

## Decision: No generics or traits on hot path

**Rationale**: Static dispatch via monomorphization eliminates dynamic dispatch overhead. But for this feature's MVP, concrete types (`u64` for price/qty, `String` for order_id) suffice. Generics add complexity without benefit at this stage.

**Alternatives considered**:
- Generic `OrderBook<P, Q>`: Zero-overhead via monomorphization but premature abstraction. Rejected for MVP.
- Trait objects (`dyn`): dynamic dispatch overhead. Rejected.
