# Feature Specification: Market Orders

**Feature Branch**: `004-market-buy-and`

**Created**: 2026-07-24

**Status**: Draft

**Input**: User description: "market buy and market sell orders are implemented - execute immediately at best available price, crossing against the opposing side until filled or book exhausted, with zero-allocation on the hot path."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Market Buy Consumes Best Asks (Priority: P1)

A trader submits a market buy order specifying only a quantity. The engine crosses the order against existing asks starting from the best (lowest) price, generating trades at each ask's price level, until the full quantity is filled or all asks are exhausted. Any unfilled quantity is discarded — market orders do not rest on the book.

**Why this priority**: Market orders are the aggressive counterpart to limit orders. Without them, a trader cannot execute immediately at market price — only passive limit orders exist. This is the foundational market order operation.

**Independent Test**: Pre-populate asks at [50000 qty 2, 51000 qty 1]. Submit a market buy for quantity 3. Verify two trades: (50000 qty 2, 51000 qty 1), both asks consumed, no residual. Verify 26 regression tests still pass.

**Acceptance Scenarios**:

1. **Given** asks at [50000 qty 3], **When** a market buy for quantity 2 is submitted, **Then** one trade is generated at 50000 qty 2 (taker is buyer), the ask is reduced to qty 1, and no residual remains.
2. **Given** asks at [50000 qty 2, 51000 qty 1], **When** a market buy for quantity 4 is submitted, **Then** two trades are generated: (50000 qty 2, 51000 qty 1), both asks are fully consumed (qty=0 in snapshot), and the remaining 1 unit of quantity is discarded (book exhausted).
3. **Given** an empty ask side, **When** a market buy for quantity 5 is submitted, **Then** no trades occur, the order is accepted with empty result, and the bid/ask sides are unchanged.

---

### User Story 2 - Market Sell Consumes Best Bids (Priority: P2)

A trader submits a market sell order. The engine crosses the order against existing bids starting from the best (highest) price. Symmetric to market buy — crosses until filled or book exhausted, discards unfilled remainder.

**Why this priority**: The sell side completes market order coverage. Without it, only buy-side market orders exist, creating an asymmetric feature set.

**Independent Test**: Pre-populate bids at [51000 qty 2, 50000 qty 3]. Submit a market sell for quantity 4. Verify two trades: (51000 qty 2, 50000 qty 2), first bid consumed, second bid reduced to 1. Discarded: 1 unit.

**Acceptance Scenarios**:

1. **Given** bids at [51000 qty 2, 50000 qty 3], **When** a market sell for quantity 4 is submitted, **Then** two trades are generated: (51000 qty 2, 50000 qty 2), first bid consumed (qty=0), second bid reduced to qty 1, 1 unit unfilled and discarded.
2. **Given** bids at [50000 qty 5], **When** a market sell for quantity 5 is submitted, **Then** one trade at 50000 qty 5, bid consumed (qty=0), no remainder, fully filled.
3. **Given** an empty bid side, **When** a market sell for quantity 10 is submitted, **Then** no trades occur, order is accepted with empty result, unchanged book.

---

### User Story 3 - Reject Invalid Market Orders (Priority: P3)

A trader submits a market order with invalid parameters. The system rejects the order before any crossing occurs, protecting the order book from malformed entries.

**Why this priority**: Input validation prevents corrupted state. Same validation pattern as limit orders from features 001 and 002, applied to the market order path.

**Independent Test**: Submit market buy with qty=0, assert Rejected with empty snapshot and no trades.

**Acceptance Scenarios**:

1. **Given** any order book state, **When** a market buy with quantity 0 is submitted, **Then** the order is rejected with Rejected status, no trades, empty book snapshot.
2. **Given** an order book containing order ID "ORD-1", **When** a market buy with the same ID is submitted, **Then** the order is rejected (cross-side uniqueness enforced).
3. **Given** any order book state, **When** a market buy with an empty order ID is submitted, **Then** the order is rejected.

---

### Edge Cases

- What happens when a market buy sweeps all asks and still has unfilled quantity? The remaining quantity is discarded — market orders do not rest on the book.
- What happens when the opposing side is empty? The order is accepted with no trades, no state change — effectively a no-op.
- What happens when a market buy encounters a price level that is partially filled (reduced from a previous crossing)? The remaining quantity at that level is available for crossing, same as with limit orders.
- What happens when a very large market order sweeps many price levels? The engine handles it through the same multi-level sweep iteration already present in the limit order crossing loop.
- How does the BookSnapshot report consumed levels? Same as limit orders — fully consumed levels appear with qty=0.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST accept a market buy order identified by a unique order ID and specifying a quantity only (no price).
- **FR-002**: System MUST accept a market sell order identified by a unique order ID and specifying a quantity only (no price).
- **FR-003**: A market buy MUST cross against existing asks starting from the best (lowest) price, generating one Trade per affected price level at that level's price, until the quantity is filled or all asks are exhausted.
- **FR-004**: A market sell MUST cross against existing bids starting from the best (highest) price, generating one Trade per affected price level at that level's price, until the quantity is filled or all bids are exhausted.
- **FR-005**: Any unfilled quantity after exhausting the opposing side MUST be discarded — market orders do not produce residual resting orders.
- **FR-006**: System MUST reject market orders with a quantity of zero.
- **FR-007**: System MUST reject market orders whose order ID already exists in the order book (cross-side uniqueness, same as limit orders).
- **FR-008**: System MUST reject market orders with an empty order ID.
- **FR-009**: The OrderResult for an accepted market order MUST contain all generated trades in execution order and a BookSnapshot reflecting net changes to the opposing side.
- **FR-010**: System MUST operate with zero runtime memory allocation on the market order hot path, reusing the existing pre-allocated trade buffer and crossing loop structure.
- **FR-011**: Existing limit order functionality (place_limit_buy, place_limit_sell) MUST remain unchanged — market orders are new methods alongside them.

### Key Entities *(include if feature involves data)*

- **Market order**: An aggressive instruction to buy or sell a specified quantity immediately at the best available prices. Has no limit price. Does not rest on the book if unfilled — remaining quantity is discarded.
- **Trade**: Same entity as 003 — records the execution price (maker's resting price), filled quantity, and taker side. Unchanged from the limit order crossing feature.
- **OrderBook**: Extended with two new methods. Crossing logic shared with limit orders — the existing multi-level sweep loop can be adapted for market orders by removing the price-gate condition.
- **OrderResult**: Same structure as 003 — contains trades list and BookSnapshot. When the opposing side is empty, trades is empty and snapshot is empty (accepted, no-op).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A market buy for a quantity fully available on the ask side generates exactly one trade per consumed price level and returns an Accepted status within a single invocation.
- **SC-002**: A market buy for a quantity exceeding available ask liquidity generates trades for all available quantity and discards the remainder — no residual order placed.
- **SC-003**: A market sell crossing multiple bid levels generates trades in correct price sequence (descending, best first) and returns an Accepted status.
- **SC-004**: 100% of existing regression tests (26 tests from features 001, 002, 003) continue to pass — limit order functionality is not affected.
- **SC-005**: 100% of invalid market orders (qty=0, duplicate ID, empty ID) are rejected without corrupting existing book state.
- **SC-006**: All acceptance scenarios are verifiable through automated tests runnable with a single command, with no external dependencies.

## Assumptions

- Market orders use the same taker-maker crossing model as limit orders: the incoming market order is the taker, trades execute at the resting (maker) order's price.
- Market orders do not place residual resting orders — any unfilled quantity after the opposing side is exhausted is discarded. This follows standard exchange semantics (market order = immediate execution, no resting).
- Cross-side order ID uniqueness is enforced: an order ID used for a limit buy cannot be reused for a market sell, and vice versa. Same shared `order_ids` tracking as limit orders.
- All prices and quantities remain unsigned 64-bit integers.
- Single trading pair. No market order price bounds or slippage protection — the order executes at whatever the book offers.
- Existing limit order methods (place_limit_buy, place_limit_sell) are not modified. Market orders are new public methods: place_market_buy and place_market_sell.
