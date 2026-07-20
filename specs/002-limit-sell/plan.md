# Implementation Plan: Limit Sell Order

**Branch**: `002-limit-sell` | **Date**: 2026-07-20 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/002-limit-sell/spec.md`

## Summary

Extend the existing OrderBook with sell-side support — a `place_limit_sell` function
symmetric to `place_limit_buy` from 001. Three user stories: insert into empty asks
(P1), aggregate into non-empty asks with ascending ordering (P2), reject invalid
inputs (P3). Cross-side order ID uniqueness enforced. No matching yet.

## Technical Context

**Language/Version**: Rust 1.75+ (stable)

**Primary Dependencies**: None on the hot path (Constitution II). Reuses existing
types (Side, OrderStatus, PriceLevel, BookSnapshot, OrderResult) from 001-limit-buy.

**Storage**: In-memory only. Adds `asks: Vec<PriceLevel>` field alongside existing
`bids` in OrderBook. Shared `order_ids: Vec<String>` for cross-side uniqueness.

**Testing**: `cargo test` — unit tests per user story. Existing 8 tests from 001
continue to pass.

**Target Platform**: Linux.

**Project Type**: Library module — extends `engine/src/order_book.rs`.

**Performance Goals**: Zero heap allocation per `place_limit_sell` call (same
strategy as 001: Vec::with_capacity, linear scan, no Box).

**Constraints**: Same as 001.

**Scale/Scope**: Single trading pair. Single-threaded.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Polylith Component Purity | N/A | |
| II. Zero-Allocation Performance | ✅ PASS | Same Vec::with_capacity strategy as 001 |
| III. Event Sourcing First | ✅ PASS | Returns structured OrderResult |
| IV. Chaos-Driven Verification | N/A | M3 |
| V. Polyglot Boundary Excellence | ✅ PASS | ZMQ-compatible |
| Contract First (Workflow) | ✅ PASS | Malli schemas in contracts/ |
| Observable Operations (Workflow) | ✅ PASS | order_id echoed in result |

### Post-Design Re-evaluation (after Phase 1)

All gates remain PASS. Design artifacts align with constitution:

- **II. Zero-Allocation**: Asks uses same Vec::with_capacity(128) strategy. Linear scan for both aggregation and ID check. No heap on hot path.
- **Contract First**: `contracts/order-book.md` updated with place_limit_sell Malli schema.
- **Observable Operations**: OrderResult echoes order_id; cross-side ID uniqueness (FR-007) enables unambiguous invoke↔complete pairing.

## Project Structure

### Documentation (this feature)

```text
specs/002-limit-sell/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── order-book.md
└── tasks.md
```

### Source Code (repository root)

```text
engine/
├── src/
│   ├── types.rs          # Unchanged (Side::Sell, OrderStatus, etc. already defined)
│   ├── order_book.rs     # EXTENDED: add asks, place_limit_sell, shared order_ids
│   └── lib.rs            # Unchanged
└── tests/
    └── order_book_test.rs # EXTENDED: 6 new tests for US1/US2/US3 (sell side)
```

**Structure Decision**: Single project. Extend existing OrderBook struct with
`asks` field and `place_limit_sell` method. Tests appended to existing test file.
No new files needed — only modifications.

## Complexity Tracking

No constitution violations. No complexity justification needed.
