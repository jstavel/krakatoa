# Feature Specification: Limit Sell Order

**Feature Branch**: `002-limit-sell`

**Created**: 2026-07-20

**Status**: Implemented

**Input**: User description: "Limit Sell Order"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Submit Limit Sell to Empty Order Book (Priority: P1)

A trader submits a limit sell order to an order book that currently contains no outstanding orders. The order is accepted and placed on the ask side at the requested price and quantity. No trades occur because there are no matching buy orders.

**Why this priority**: This is the foundational sell-side operation. Without the ability to place a sell order into an empty book, the order book is incomplete — only one side exists. It extends the core data model with the ask side symmetrically to bids from 001-limit-buy.

**Independent Test**: Create a fresh empty order book. Submit one limit sell order with a known order ID, price, and quantity. Verify the returned result confirms acceptance with no trades, and the ask side snapshot contains exactly one price level with the submitted price and quantity.

**Acceptance Scenarios**:

1. **Given** an empty order book with no outstanding bids or asks, **When** a trader submits a limit sell order with price 50000 and quantity 1, **Then** the order is accepted, zero trades are returned, and the ask side snapshot shows a single price level at 50000 with total quantity 1.

---

### User Story 2 - Submit Limit Sell to Non-Empty Order Book (Priority: P2)

A trader submits a limit sell order to an order book that already contains outstanding asks. If an existing ask price level matches the new order's price, the quantities are aggregated. The ask side always maintains ascending price order (best ask = lowest price).

**Why this priority**: After the empty-book case, the next natural step is placing orders into an active book. Price aggregation and correct ordering are essential for subsequent matching logic — the engine must always know which ask is the best (lowest) price.

**Independent Test**: Pre-populate the book with a limit sell at price 50000, quantity 1. Submit a second limit sell at the same price, quantity 2. Verify the ask side shows a single price level at 50000 with total quantity 3 (aggregated), and no trades occur.

**Acceptance Scenarios**:

1. **Given** an order book with one ask at price 50000 quantity 1, **When** a second limit sell at price 50000 quantity 2 is submitted, **Then** the order is accepted with no trades, and the ask side snapshot shows one price level at 50000 with quantity 3.
2. **Given** an order book with asks at prices 50000 and 52000, **When** a limit sell at price 51000 is submitted, **Then** the ask side snapshot shows three levels in ascending order: 50000, 51000, 52000.

---

### User Story 3 - Reject Invalid Limit Sell Orders (Priority: P3)

A trader submits a limit sell order with invalid parameters. The system rejects the order with a clear rejection status, protecting the order book from malformed or duplicate entries.

**Why this priority**: Input validation prevents corrupted state and ensures data integrity. Identical validation logic to limit-buy, applied to the sell side.

**Independent Test**: Submit orders with zero price, zero quantity, and a duplicate order ID. Verify each returns a Rejected status with no side effects on the order book (no asks added, no trades produced, empty book snapshot).

**Acceptance Scenarios**:

1. **Given** any order book state, **When** a limit sell order with price 0 is submitted, **Then** the order is rejected with Rejected status.
2. **Given** any order book state, **When** a limit sell order with quantity 0 is submitted, **Then** the order is rejected with Rejected status.
3. **Given** an order book containing an order with ID "ORD-1", **When** another limit sell order with the same ID "ORD-1" is submitted, **Then** the order is rejected with Rejected status.

---

### Edge Cases

- What happens when a price or quantity exceeds the maximum value the system can represent?
- What happens when an order with an empty order ID string is submitted?
- How does the book snapshot behave when the book is empty and all asks visible?
- How does the system handle order IDs that collide between bids and asks (e.g., "ORD-1" exists on bid side, then submitted as sell)?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST accept a limit sell order identified by a unique order ID, specifying a price and a quantity.
- **FR-002**: System MUST return an order result containing the submitted order ID, an acceptance status (Accepted or Rejected), a list of trades (empty for this feature), and a snapshot of changed price levels on the ask side. On rejection, the snapshot MUST be empty (no asks changed).
- **FR-003**: System MUST maintain ask price levels sorted in ascending order (lowest ask first).
- **FR-004**: System MUST aggregate quantities when a new order matches an existing ask price level.
- **FR-005**: System MUST reject orders with a price of zero.
- **FR-006**: System MUST reject orders with a quantity of zero.
- **FR-007**: System MUST reject orders whose order ID already exists anywhere in the order book (both bids and asks).
- **FR-008**: For this feature, system MUST NOT perform any matching against the bid side — all submitted limit sell orders are accepted or rejected solely on ask-side logic.

### Key Entities

- **Order**: Represents a single buy or sell instruction. Key attributes: unique order identifier, price, quantity, side (buy/sell). Side is already defined in the codebase; this feature uses `Side::Sell`.
- **OrderResult**: The response returned after processing a limit sell order. Contains: the order ID echoed back, an acceptance/rejection status, any trades produced (always empty for this feature), and a snapshot of price levels changed on the ask side.
- **OrderStatus**: Enumeration describing the outcome — Accepted (order placed) or Rejected (order denied due to validation failure). Already defined.
- **PriceLevel**: A single price point on one side of the book. Contains the price value and the total quantity aggregated at that price. Already defined.
- **BookSnapshot**: A view of changed price levels after an operation. Contains lists of modified bid levels and ask levels (bids always empty for this feature). Already defined.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A limit sell order submitted to an empty book produces a correct result (Accepted, zero trades, correct ask snapshot) within one invocation.
- **SC-002**: Order book maintains correct ask-side ordering after any sequence of valid limit sell insertions — the lowest price is always at the top of the ask list.
- **SC-003**: 100% of invalid input cases (zero price, zero quantity, duplicate ID) are rejected without corrupting existing book state.
- **SC-004**: Order IDs are unique across both bids and asks — a duplicate from either side is rejected.
- **SC-005**: All acceptance scenarios can be verified by automated tests runnable with a single command without external dependencies.

## Assumptions

- Order prices and quantities are represented as unsigned integers.
- The order book starts empty and is initialized per-test or per-session.
- The bid side is out of scope for this feature — matching between bids and asks will be handled in a subsequent feature.
- Market orders, order cancellation, and order modification are out of scope.
- The system operates on a single trading pair.
- An empty order ID is treated as invalid and should be rejected.
- Codebase already defines Side::Sell, OrderStatus, PriceLevel, BookSnapshot, and OrderResult — this feature extends OrderBook with the ask side and adds place_limit_sell.
- FR-007 extends the "unique ID" constraint from 001-limit-buy to cover both sides: an order ID used on bids cannot be reused on asks, and vice versa.
