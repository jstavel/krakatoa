# Implementation Plan: Order Matching (Crossing Engine)

**Branch**: `003-aktualni-orderbook-slouzi` | **Date**: 2026-07-23 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/003-order-matching/spec.md`

## Summary

Extend the existing `OrderBook`'s `place_limit_buy` and `place_limit_sell` with active taker-maker crossing. When an incoming order's price meets or exceeds the best opposing price, the engine crosses the incoming order against resting orders at the resting (maker) price. Partial fills leave residual orders. Multi-level sweeps consume price levels iteratively. Trade records are generated with price, quantity, and taker side. Existing validation and zero-allocation strategy preserved.

## Technical Context

**Language/Version**: Rust 1.75+ (stable)

**Primary Dependencies**: None on the crossing hot path. Reuses existing types (Side, OrderStatus, PriceLevel, BookSnapshot, OrderResult) from 001/002.

**Storage**: In-memory only. Extends existing `OrderBook` struct — adds `trades_buf: Vec<Trade>` pre-allocated with `with_capacity(128)`. No persistent storage.

**Testing**: `cargo test` — unit tests per user story. Existing 16 tests from 001 and 002 continue to pass (regression requirement).

**Target Platform**: Linux.

**Project Type**: Library module — modifies `engine/src/order_book.rs` (crossing logic), `engine/src/types.rs` (Trade fields).

**Performance Goals**: Zero runtime memory allocation in the crossing loop (in-place mutations on bids/asks vectors). One allocation per order for result construction (Trade Vec clone). Linear O(n) scan of opposing side — same strategy as 001/002.

**Constraints**: Same as 001/002 — single-threaded, single trading pair, u64 prices/quantities. Max 128 trades per order (pre-allocated buffer). Crossing loop: no `Box`, no `String`, no `HashMap`, no additional `Vec::push` reallocation beyond pre-allocated capacity.

**Scale/Scope**: Single trading pair. No matching across pairs.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Polylith Component Purity | N/A | No Clojure changes in this feature |
| II. Zero-Allocation Performance | ✅ PASS | Pre-allocated `trades_buf` with `with_capacity(128)`; crossing loop uses in-place mutations on `bids`/`asks`; single `clone()` for result. No per-trade allocation. |
| III. Event Sourcing First | ✅ PASS | Trade records returned in `OrderResult.trades` — available for Kafka logging by gateway |
| IV. Chaos-Driven Verification | N/A | M3 milestone |
| V. Polyglot Boundary Excellence | ✅ PASS | Existing ZMQ REP in `main.rs` unchanged |
| Contract First (Workflow) | ✅ PASS | Malli schemas updated in `contracts/order-book.md` |
| Observable Operations (Workflow) | ✅ PASS | Each Trade includes `taker_side`, trades ordered by execution sequence, order_id echoed in result |

### Post-Design Re-evaluation (after Phase 1)

All gates remain PASS. Design artifacts align with constitution:

- **II. Zero-Allocation**: `trades_buf` pre-allocated in `OrderBook::new()` with `with_capacity(128)`. Per-operation: `clear()` + `push()` during crossing + single `clone()` for result. Inner loop: `iter_mut()` over opposing side, in-place qty mutations, `Vec::remove(0)` for fully consumed levels. No heap in hot loop.
- **Contract First**: `contracts/order-book.md` updated with crossing semantics and Trade schema.
- **Observable Operations**: Each Trade record carries `taker_side` for unambiguous invoke↔complete pairing.

## Project Structure

### Documentation (this feature)

```text
specs/003-order-matching/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── order-book.md
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
engine/
├── src/
│   ├── types.rs          # MODIFIED: Trade struct gets fields (price, qty, taker_side)
│   ├── order_book.rs     # EXTENDED: crossing logic in both methods, trades_buf field
│   ├── lib.rs            # Unchanged
│   └── main.rs           # Unchanged
└── tests/
    └── order_book_test.rs # EXTENDED: ~12 new tests for US1/US2/US3 crossing scenarios
```

**Structure Decision**: Single project. Modify existing `engine/src/types.rs` (Trade struct) and `engine/src/order_book.rs` (crossing logic). Tests appended to existing test file. No new files — only modifications to existing.

## Complexity Tracking

No constitution violations. No complexity justification needed.
