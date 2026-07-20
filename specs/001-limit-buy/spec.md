# Feature Specification: Limit Buy Order

**Feature Branch**: `001-limit-buy`

**Created**: 2026-07-16

**Status**: Implemented

**Input**: User description: "LimitBuyOrder"

## Clarifications

### Session 2026-07-20

- Q: Does Order entity need a Side attribute now? → A: Yes — Order has Side field immediately, even though only Buy is used in this feature.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Submit Limit Buy to Empty Order Book (Priority: P1)

A trader submits a limit buy order to an order book that currently contains no outstanding orders. The order is accepted and placed on the bid side at the requested price and quantity. No trades occur because there are no matching sell orders.

**Why this priority**: This is the foundational operation of the matching engine. Without the ability to place an order into an empty book, no other order types or matching logic can function. It validates the core data model and order book structure.

**Independent Test**: Create a fresh empty order book. Submit one limit buy order with a known order ID, price, and quantity. Verify the returned result confirms acceptance with no trades, and the bid side snapshot contains exactly one price level with the submitted price and quantity.

**Acceptance Scenarios**:

1. **Given** an empty order book with no outstanding bids or asks, **When** a trader submits a limit buy order with price 50000 and quantity 1, **Then** the order is accepted, zero trades are returned, and the bid side snapshot shows a single price level at 50000 with total quantity 1.
2. **Given** an empty order book, **When** two limit buy orders with different prices (e.g., 51000 and 50000) are submitted sequentially, **Then** both orders are accepted and the bid side snapshot lists price levels in descending order (51000 above 50000).

---

### User Story 2 - Submit Limit Buy to Non-Empty Order Book (Priority: P2)

A trader submits a limit buy order to an order book that already contains outstanding bids. If an existing bid price level matches the new order's price, the quantities are aggregated. The bid side always maintains descending price order (best bid = highest price).

**Why this priority**: After the empty-book case, the next natural step is placing orders into an active book. Price aggregation and correct ordering are essential for subsequent matching logic — the engine must always know which bid is the best (highest) price.

**Independent Test**: Pre-populate the book with a limit buy at price 50000, quantity 1. Submit a second limit buy at the same price, quantity 2. Verify the bid side shows a single price level at 50000 with total quantity 3 (aggregated), and no trades occur.

**Acceptance Scenarios**:

1. **Given** an order book with one bid at price 50000 quantity 1, **When** a second limit buy at price 50000 quantity 2 is submitted, **Then** the order is accepted with no trades, and the bid side snapshot shows one price level at 50000 with quantity 3.
2. **Given** an order book with bids at prices 50000 and 48000, **When** a limit buy at price 49000 is submitted, **Then** the bid side snapshot shows three levels in descending order: 50000, 49000, 48000.

---

### User Story 3 - Reject Invalid Limit Buy Orders (Priority: P3)

A trader submits a limit buy order with invalid parameters. The system rejects the order with a clear rejection status, protecting the order book from malformed or duplicate entries.

**Why this priority**: Input validation prevents corrupted state and ensures data integrity. While traders using well-formed clients will rarely hit these paths, a robust engine must handle them explicitly rather than silently corrupting state or crashing.

**Independent Test**: Submit orders with zero price, zero quantity, and a duplicate order ID. Verify each returns a Rejected status with no side effects on the order book (no bids added, no trades produced, book snapshot unchanged).

**Acceptance Scenarios**:

1. **Given** any order book state, **When** a limit buy order with price 0 is submitted, **Then** the order is rejected with Rejected status.
2. **Given** any order book state, **When** a limit buy order with quantity 0 is submitted, **Then** the order is rejected with Rejected status.
3. **Given** an order book containing an order with ID "ORD-1", **When** another limit buy order with the same ID "ORD-1" is submitted, **Then** the order is rejected with Rejected status.

---

### Edge Cases

- What happens when a price or quantity exceeds the maximum value the system can represent?
- What happens when an order with an empty order ID string is submitted?
- How does the book snapshot behave when the book is empty and all bids visible?
- What happens when the same trader submits many orders at incrementally different prices — does the book maintain correct sort order at scale?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST accept a limit buy order identified by a unique order ID, specifying a price and a quantity.
- **FR-002**: System MUST return an order result containing the submitted order ID, an acceptance status (Accepted or Rejected), a list of trades (empty for this feature), and a snapshot of changed price levels on the bid side. On rejection, the snapshot MUST be empty (no bids changed).
- **FR-003**: System MUST maintain bid price levels sorted in descending order (highest bid first).
- **FR-004**: System MUST aggregate quantities when a new order matches an existing bid price level.
- **FR-005**: System MUST reject orders with a price of zero.
- **FR-006**: System MUST reject orders with a quantity of zero.
- **FR-007**: System MUST reject orders whose order ID already exists in the order book.
- **FR-008**: For this feature, system MUST NOT perform any matching against the ask side — all submitted limit buy orders are accepted or rejected solely on bid-side logic.

### Key Entities

- **Order**: Represents a single buy or sell instruction. Key attributes: unique order identifier, price, quantity, side (buy/sell — only Buy is used in this feature, but Sell is defined for future use).
- **OrderResult**: The response returned after processing a limit buy order. Contains: the order ID echoed back, an acceptance/rejection status, any trades produced (always empty for this feature), and a snapshot of price levels changed on the bid side.
- **OrderStatus**: Enumeration describing the outcome — Accepted (order placed) or Rejected (order denied due to validation failure).
- **PriceLevel**: A single price point on one side of the book. Contains the price value and the total quantity aggregated at that price.
- **BookSnapshot**: A view of changed price levels after an operation. Contains lists of modified bid levels and ask levels (asks always empty for this feature).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A limit buy order submitted to an empty book produces a correct result (Accepted, zero trades, correct bid snapshot) within one invocation — no retries or state leakage between calls.
- **SC-002**: Order book maintains correct bid-side ordering after any sequence of valid limit buy insertions — the highest price is always at the top of the bid list.
- **SC-003**: 100% of invalid input cases (zero price, zero quantity, duplicate ID) are rejected without corrupting existing book state.
- **SC-004**: All acceptance scenarios can be verified by automated tests runnable with a single command without external dependencies.

## Assumptions

- Order prices and quantities are represented as unsigned integers (whole units, no fractional amounts needed for this feature).
- The order book starts empty and is initialized per-test or per-session — no persistent storage between invocations for this feature.
- The ask side (sell orders) is out of scope for this feature — matching between bids and asks will be handled in a subsequent feature.
- Market orders, order cancellation, and order modification are out of scope for this feature.
- The system operates on a single trading pair — multi-pair order books are out of scope.
- An empty order ID (zero-length string) is treated as invalid and should be rejected (extension of P3 validation).
