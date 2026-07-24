# Feature Specification: Order Matching (Crossing Engine)

**Feature Branch**: `003-aktualni-orderbook-slouzi`

**Created**: 2026-07-23

**Status**: Draft

**Input**: User description: "Aktualni OrderBook slouzi jako pasivni uloziste a nemumi parovat obchody, coz blokuje hlavni ucel matching enginu. Je potreba rozsirit place_limit_buy a place_limit_sell o aktivni crossing, podporu partial fills a generovani realnych Trades zaznamu pri zachovani zero-allocation pravidel."

## Clarifications

### Session 2026-07-23

- Q: Should zero-quantity price levels (fully consumed by crossing) appear in the BookSnapshot? → A: Yes — removed levels appear in BookSnapshot with qty=0 so consumers can verify both partial reductions and full removals.
- Q: Residual order price — should it be incoming order's price (FR-009) or last crossed resting price (US3/AC2-3)? → A: Incoming order's price per FR-009. Fixed US3/AC2 residual to 53000, US3/AC3 residual to 49000.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Buy Order Crosses Single Ask at Matching Price (Priority: P1)

A trader submits a limit buy order at a price that meets or exceeds the lowest (best) ask price. The engine crosses the incoming buy against the resting ask at the ask's price, generating a trade. If the quantities match exactly, both sides are fully consumed and no residual order remains in the book.

**Why this priority**: This is the fundamental operation that transforms the OrderBook from a passive data store into a functioning matching engine. Without crossing at a single matching price, the engine cannot produce any trades, which is its core purpose.

**Independent Test**: Pre-populate an order book with one ask at price 50000, quantity 2. Submit a limit buy at price 50000, quantity 2. Verify that one trade is generated with price 50000, quantity 2, the ask side is empty, no residual buy order remains, and the order is accepted.

**Acceptance Scenarios**:

1. **Given** an order book with one ask at price 50000 quantity 3, 
   **When** a limit buy at price 51000 quantity 3 is submitted, 
   **Then** one trade is generated at price 50000 quantity 3 (crossed at resting order price), the ask side is empty, no residual remains, and the order status is Accepted.
   
2. **Given** an order book with one ask at price 50000 quantity 2, 
   **When** a limit buy at price 49000 quantity 2 is submitted, 
   **Then** no trade occurs (buy price below best ask), the buy order is placed on the bid side at price 49000, and the ask side is unchanged.

---

### User Story 2 - Partial Fill with Residual Order (Priority: P2)

A trader submits a limit order that crosses an existing order but the quantities do not match exactly. The engine executes a partial trade for the overlapping quantity, and the remainder stays in the order book as a resting order.

**Why this priority**: Partial fills are the norm in real markets — exact quantity matches are rare. Without partial fill support, the engine would either over-fill (incorrect) or reject partially matchable orders (unusable). This extends the P1 single-match case to handle quantity mismatches.

**Independent Test**: Pre-populate an order book with one ask at price 50000, quantity 2. Submit a limit buy at price 51000, quantity 5. Verify one trade at price 50000, quantity 2 (partial fill), no remaining ask, and a residual buy order of quantity 3 is placed on the bid side at price 51000.

**Acceptance Scenarios**:

1. **Given** an order book with one ask at price 50000 quantity 2, **When** a limit buy at price 51000 quantity 5 is submitted (buy qty > ask qty), **Then** one trade is generated at price 50000 quantity 2, the ask is fully consumed, and a residual buy order of price 51000 quantity 3 is placed on the bid side.
2. **Given** an order book with one ask at price 50000 quantity 5, **When** a limit buy at price 51000 quantity 2 is submitted (buy qty < ask qty), **Then** one trade is generated at price 50000 quantity 2, the ask quantity is reduced to 3 (resting order partially filled), and no residual buy order remains.
3. **Given** an order book with one bid at price 50000 quantity 5, **When** a limit sell at price 49000 quantity 2 is submitted (sell crosses bid, sell qty < bid qty), **Then** one trade is generated at price 50000 quantity 2, the bid quantity is reduced to 3, and no residual sell order remains.

---

### User Story 3 - Multi-Level Sweep (Priority: P3)

A trader submits a large limit buy order whose quantity is sufficient to consume multiple ask price levels. The engine walks up the ask side from the best (lowest) price upward, generating a trade at each level until the incoming order is fully filled or all crossing asks are exhausted. Any remaining quantity after sweeping all possible levels is placed as a resting order.

**Why this priority**: Large orders that sweep multiple price levels represent real market behavior and must be handled correctly for the engine to be considered complete. However, this is an extension of the single-crossing and partial-fill logic — the underlying crossing mechanic is the same, just applied iteratively.

**Independent Test**: Pre-populate an order book with asks at price 50000 quantity 2 and price 52000 quantity 3. Submit a limit buy at price 53000 quantity 4. Verify two trades: first at 50000 qty 2, second at 52000 qty 2 (partial fill of second level), and the residual ask at 52000 has quantity 1 remaining.

**Acceptance Scenarios**:

1. **Given** asks at [50000 qty 2, 52000 qty 3], **When** a limit buy at price 53000 quantity 4 is submitted, **Then** two trades are generated: (50000 qty 2, 52000 qty 2), the 50000 ask is fully consumed, the 52000 ask remains with quantity 1, and no residual buy remains.
2. **Given** asks at [50000 qty 1, 51000 qty 1, 52000 qty 1], **When** a limit buy at price 53000 quantity 5 is submitted, **Then** three trades are generated consuming all three asks, and a residual buy of price 53000 quantity 2 is placed on the bid side.
3. **Given** bids at [51000 qty 2, 50000 qty 3], **When** a limit sell at price 49000 quantity 6 is submitted, **Then** two trades are generated (51000 qty 2, 50000 qty 3), both bids are fully consumed, and a residual sell of price 49000 quantity 1 is placed on the ask side.

---

### Edge Cases

- What happens when a crossing would reduce an existing price level to exactly zero quantity? The zero-quantity level must be removed from the book entirely and reported in the BookSnapshot with qty=0 (for change traceability).
- What happens when an order is fully filled across multiple levels with no residual? The order status is Accepted with the trades list populated and no resting order placed.
- What happens when a buy order price exactly equals the best ask price? The trade executes at that shared price (the resting order's price, which equals the incoming order's price).
- What happens when the opposing side of the book is empty? The order is placed as a resting order without crossing — existing behavior preserved.
- What happens when crossing a previously aggregated price level (multiple orders at same price)? The entire aggregated quantity at that price level is available for crossing, following price-time priority within the level.
- What happens when a crossing order has the same order ID as an existing filled order? The order ID remains unique per the existing validation — previous matching does not affect ID uniqueness (order IDs are never removed from the tracking list after fills).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST cross an incoming limit buy order against existing asks when the buy price is greater than or equal to the best (lowest) ask price.
- **FR-002**: System MUST cross an incoming limit sell order against existing bids when the sell price is less than or equal to the best (highest) bid price.
- **FR-003**: System MUST generate a Trade record for each matched quantity at the price of the resting order (maker price), not the incoming order price.
- **FR-004**: Each Trade record MUST include at minimum: the execution price, the filled quantity, and the side of the taker (the incoming order).
- **FR-005**: System MUST support partial fills — when incoming order quantity exceeds or falls short of the resting order quantity, the excess is handled as a residual resting order or quantity reduction.
- **FR-006**: System MUST remove a price level from the book when its quantity reaches zero after crossing, and MUST report the removed level in the BookSnapshot with qty=0.
- **FR-007**: System MUST iterate through multiple price levels on the opposing side when the incoming order quantity is large enough to consume more than one level (multi-level sweep).
- **FR-008**: When an incoming order is fully filled (no residual quantity remains), the system MUST NOT place a resting order on the book.
- **FR-009**: When an incoming order is only partially filled, the residual quantity MUST be placed on the appropriate side of the book at the incoming order's price.
- **FR-010**: After crossing, the order book MUST maintain correct price ordering: bids descending (highest first), asks ascending (lowest first).
- **FR-011**: The OrderResult MUST contain all trades generated during crossing in the order they were executed (ascending ask price for buys, descending bid price for sells).
- **FR-012**: Existing validation rules (price=0, qty=0, empty/duplicate order ID rejection) MUST remain unchanged and apply before any crossing logic.
- **FR-013**: System MUST operate with zero runtime memory allocation on the crossing hot path — no heap allocation per trade generation, reusing pre-allocated structures.

### Key Entities *(include if feature involves data)*

- **Trade**: Record of an executed match between a taker (incoming order) and a maker (resting order). Key attributes: execution price (matching the resting order's price), filled quantity, and the side of the taker (Buy or Sell). Replaces the previously empty Trade struct.
- **OrderBook**: Extended with crossing logic. Maintains bids (descending) and asks (ascending). After crossing, consumed quantities are removed and zero-quantity levels are pruned.
- **OrderResult**: Extended to contain a non-empty trades list when crossing occurs. The trades field now carries the chronological sequence of executed trades. The book_snap reflects net changes to both sides after all crossing and residual placement, with fully consumed levels reported as qty=0. Bids_changed and asks_changed list levels that were added, modified, or removed.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A crossing order that fully matches a single resting order generates exactly one trade and returns an Accepted status within a single invocation.
- **SC-002**: A crossing order that partially fills a resting order correctly produces one trade, adjusts the resting order quantity, and handles the residual quantity (either placed or discarded) without corrupting the order book state.
- **SC-003**: A large crossing order sweeping three or more price levels generates one trade per affected level in correct price sequence, with each intermediate level correctly consumed or reduced.
- **SC-004**: 100% of existing regression tests (16 tests from features 001 and 002) continue to pass — non-crossing scenarios (empty opposing side, below-threshold price) behave identically to before.
- **SC-005**: Zero-quantity price levels are never observable in the order book state after any sequence of valid order submissions.
- **SC-006**: All acceptance scenarios are verifiable through automated tests runnable with a single command, with no external dependencies.

## Assumptions

- Crossing uses a taker-maker model: the incoming order is the taker and trades execute at the resting (maker) order's price, which is always equal to or better than the taker's limit price.
- Within a single price level, orders fill on a first-come-first-served (FIFO) basis. Since individual orders within an aggregated level are not tracked separately (only total quantity is stored), the entire aggregated quantity at that level is available for crossing.
- The existing order ID uniqueness constraint remains: once an order ID is used (whether filled or resting), it cannot be reused. Order IDs are not removed after the order is fully filled — the tracking list is append-only.
- All prices and quantities remain unsigned 64-bit integers. No fractional quantities.
- Matching engine continues to operate on a single trading pair.
- Market orders, order cancellation, and order modification remain out of scope.
- No trading fees, commissions, or regulatory reporting are included in this feature.
- Self-trade prevention is out of scope for this feature — the system assumes a single entity does not trade against itself.
