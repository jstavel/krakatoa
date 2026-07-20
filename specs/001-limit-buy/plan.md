# Implementation Plan: Limit Buy Order

**Branch**: `001-limit-buy` | **Date**: 2026-07-20 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/001-limit-buy/spec.md`

## Summary

Implement a zero-allocation order book capable of accepting limit buy orders.
Three user stories: insert into empty book (P1), aggregate into non-empty book
with descending price ordering (P2), and reject invalid inputs (P3). No
matching against asks — this feature establishes the core data model and bid-side
state management that all subsequent features build upon.

## Technical Context

**Language/Version**: Rust 1.75+ (stable)

**Primary Dependencies**: None on the hot path (Constitution II — zero-allocation).
Only standard library types reused via pre-allocated structures where possible.

**Storage**: In-memory only. No persistence in this feature (assumption per spec).

**Testing**: `cargo test` — unit tests per user story. Each acceptance scenario
maps to a test function.

**Target Platform**: Linux (KVM VM per Constitution constraints in production;
local development on host for this feature).

**Project Type**: Library module — `engine/src/order_book.rs`. Consumed by
`engine/src/main.rs` (existing REP socket skeleton).

**Performance Goals**: Zero heap allocation per `place_limit_buy` call.
Deterministic latency, no garbage collection pauses.

**Constraints**: Constitution II (Zero-Allocation), Constitution 1.2.0
(Observable Operations — echo order_id), Constitution 1.2.0 (Contract First —
Malli schemas in contracts/).

**Scale/Scope**: Single trading pair, single-threaded. Concurrent access
deferred to Milestone 3.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle                        | Status  | Notes                                                                                      |
|----------------------------------|---------|--------------------------------------------------------------------------------------------|
| I. Polylith Component Purity     | N/A     | Engine feature, not Clojure component                                                      |
| II. Zero-Allocation Performance  | ✅ PASS | No `Box`, `Vec::push` on hot path. Fixed-capacity structures, pre-allocated.               |
| III. Event Sourcing First        | ✅ PASS | Engine returns structured result; gateway appends to Kafka (out of scope for engine)       |
| IV. Chaos-Driven Verification    | N/A     | Jepsen in M3. Unit tests for correctness in M2.                                            |
| V. Polyglot Boundary Excellence  | ✅ PASS | Engine communicates via ZMQ (existing skeleton). OrderResult is serializable byte payload. |
| Contract First (Workflow)        | ✅ PASS | Malli schemas in contracts/ before implementation.                                         |
| Observable Operations (Workflow) | ✅ PASS | OrderResult echoes order_id. Gateway observes via Kafka append.                            |

### Post-Design Re-evaluation (after Phase 1)

All gates remain PASS. Design artifacts (research.md, data-model.md, contracts/) align with constitution:

- **II. Zero-Allocation**: Research confirms Vec::with_capacity + linear scan, zero heap allocation on hot path.
- **Contract First**: `contracts/order-book.md` defines Malli schema for place_limit_buy → OrderResult.
- **Observable Operations**: OrderResult echoes order_id; BookSnapshot is empty on rejection per clarifications.

## Project Structure

### Documentation (this feature)

```text
specs/001-limit-buy/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output — Malli schemas
│   └── order-book.md
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
engine/
├── Cargo.toml
├── src/
│   ├── main.rs          # Existing REP socket skeleton (unchanged this feature)
│   ├── order_book.rs    # NEW: OrderBook, Order, OrderResult, PriceLevel
│   └── types.rs         # NEW: Shared type definitions (Side, OrderStatus, etc.)
└── tests/
    └── order_book_test.rs  # NEW: Unit tests for US1, US2, US3
```

**Structure Decision**: Single project (Rust library). Types in `types.rs`,
core logic in `order_book.rs`, tests in `tests/order_book_test.rs`. All
zero-allocation — no external crates beyond what `zmq` provides for main.rs.

## Complexity Tracking

No constitution violations. No complexity justification needed.
