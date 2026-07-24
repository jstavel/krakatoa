# Tasks: Order Matching (Crossing Engine)

**Input**: Design documents from `/specs/003-order-matching/`

**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Organization**: Tasks grouped by user story. Extends existing code — modifies `engine/src/types.rs`, `engine/src/order_book.rs`, `engine/tests/order_book_test.rs`.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- Engine code: `engine/src/`
- Tests: `engine/tests/`

---

## Phase 1: Setup

**Purpose**: Verify baseline before any modifications.

- [x] T001 Verify `cargo test` passes with all 16 existing tests before modification

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Expand Trade struct, add pre-allocated trade buffer, implement crossing loop in both methods. The crossing loop inherently supports all three user stories (exact match, partial fill, multi-level sweep) — this phase contains the complete implementation.

- [x] T002 [P] Expand `Trade` struct in `engine/src/types.rs` — replace empty struct with fields `pub price: u64`, `pub qty: u64`, `pub taker_side: Side`. Update derives to include `Copy` alongside existing `Debug, Clone, PartialEq`.
- [x] T003 Add `trades_buf: Vec<Trade>` field to `OrderBook` struct in `engine/src/order_book.rs` — initialize with `Vec::with_capacity(128)` in `new()`. Add `use crate::types::Trade;` to imports.
- [x] T004 Implement crossing loop in `place_limit_buy` in `engine/src/order_book.rs` — after existing validation (before the bid insertion block): add `self.trades_buf.clear()`, then loop while `self.asks` is non-empty and `price >= self.asks[0].price` and `remaining_qty > 0`: if `asks[0].qty <= remaining_qty`, push Trade at ask price, push `PriceLevel { ask.price, qty: 0 }` to asks_changed snapshot, subtract qty, remove asks[0]; else push Trade at ask price with remaining_qty, reduce ask qty, push updated ask to snapshot, remaining_qty = 0, break. After loop: if remaining_qty > 0, place residual on bids at incoming price per existing insertion logic; else do not place resting order. Push order_id, return Accepted with `self.trades_buf.clone()` and populated book_snap.
- [x] T005 Implement crossing loop in `place_limit_sell` in `engine/src/order_book.rs` — symmetric to T004: after validation, `self.trades_buf.clear()`, loop while `self.bids` is non-empty and `price <= self.bids[0].price` and `remaining_qty > 0`: cross against bids at maker price, push Trade with `taker_side: Side::Sell`, track bids_changed with qty=0 for fully consumed levels, residual on asks at incoming price if remaining_qty > 0.

**Checkpoint**: `cargo build` compiles. OrderBook has trades_buf field. Both place_limit_* methods have crossing logic. Existing 16 tests still pass (crossing not triggered when opposing side is empty).

---

## Phase 3: User Story 1 - Buy Order Crosses Single Ask at Matching Price (Priority: P1) 🎯 MVP

**Goal**: Verify buy crossing against a single ask level — exact quantity match, price above ask, and price exactly equals ask. Also verify no-crossing when buy price is below best ask.

**Independent Test**: Pre-populate book with ask at 50000 qty 3, submit buy at 51000 qty 3, assert 1 trade at 50000 qty 3, ask fully consumed (qty=0 in snapshot), no residual.

### Tests for User Story 1

> Write these tests FIRST, ensure they FAIL before implementation is complete

- [x] T006 [P] [US1] Write unit test `test_cross_buy_single_ask_exact_match` in `engine/tests/order_book_test.rs` — pre-populate ask at 50000 qty 3, submit `place_limit_buy("ORD-B1", 51000, 3)`, assert status=Accepted, trades list has exactly 1 Trade with price=50000 qty=3 taker_side=Buy, asks_changed=[PriceLevel{50000, 0}], bids_changed empty
- [x] T007 [P] [US1] Write unit test `test_no_cross_buy_below_ask` in `engine/tests/order_book_test.rs` — pre-populate ask at 50000 qty 2, submit `place_limit_buy("ORD-B2", 49000, 2)`, assert status=Accepted, trades list empty, buy placed on bids at 49000 (bids_changed=[PriceLevel{49000, 2}]), ask unchanged
- [x] T008 [P] [US1] Write unit test `test_cross_buy_ask_exact_price` in `engine/tests/order_book_test.rs` — pre-populate ask at 50000 qty 2, submit `place_limit_buy("ORD-B3", 50000, 2)`, assert 1 trade at 50000 qty 2 (price equality), ask consumed (qty=0), no residual

**Checkpoint**: `cargo test` — 16 existing + 3 new US1 tests pass. Buy crossing verified for exact match, below-ask no-cross, and price equality.

---

## Phase 4: User Story 2 - Partial Fill with Residual Order (Priority: P2)

**Goal**: Verify partial fills — when buy qty ≠ ask qty, the engine correctly handles the residual (either placed on bids or reducing rest order). Also verify sell-side crossing with single bid.

**Independent Test**: Ask at 50000 qty 2, buy at 51000 qty 5, assert 1 trade at 50000 qty 2, ask consumed, residual buy at 51000 qty 3 on bids.

### Tests for User Story 2

- [x] T009 [P] [US2] Write unit test `test_partial_fill_buy_larger_than_ask` in `engine/tests/order_book_test.rs` — pre-populate ask at 50000 qty 2, submit `place_limit_buy("ORD-B4", 51000, 5)`, assert 1 trade at 50000 qty 2 (taker_side=Buy), asks_changed=[PriceLevel{50000,0}], bids_changed=[PriceLevel{51000,3}] (residual at incoming price)
- [x] T010 [P] [US2] Write unit test `test_partial_fill_buy_smaller_than_ask` in `engine/tests/order_book_test.rs` — pre-populate ask at 50000 qty 5, submit `place_limit_buy("ORD-B5", 51000, 2)`, assert 1 trade at 50000 qty 2, asks_changed=[PriceLevel{50000,3}] (reduced), bids_changed empty, no residual
- [x] T011 [P] [US2] Write unit test `test_partial_fill_sell_smaller_than_bid` in `engine/tests/order_book_test.rs` — pre-populate bid at 50000 qty 5, submit `place_limit_sell("ORD-S1", 49000, 2)`, assert 1 trade at 50000 qty 2 (taker_side=Sell), bids_changed=[PriceLevel{50000,3}] (reduced), asks_changed empty, no residual
- [x] T012 [P] [US2] Write unit test `test_partial_fill_sell_larger_than_bid` in `engine/tests/order_book_test.rs` — pre-populate bid at 50000 qty 2, submit `place_limit_sell("ORD-S2", 49000, 5)`, assert 1 trade at 50000 qty 2, bids_changed=[PriceLevel{50000,0}], asks_changed=[PriceLevel{49000,3}] (residual at incoming price)

**Checkpoint**: `cargo test` — 19 existing + 4 new US2 tests pass. Partial fills verified for both buy and sell sides.

---

## Phase 5: User Story 3 - Multi-Level Sweep (Priority: P3)

**Goal**: Verify that a large order sweeps multiple price levels, generating one trade per affected level. Also verify multi-level sweep with residual.

**Independent Test**: Asks at [50000 qty 2, 52000 qty 3], buy at 53000 qty 4, assert 2 trades: (50000,2) and (52000,2), 50000 ask consumed, 52000 ask reduced to 1.

### Tests for User Story 3

- [x] T013 [P] [US3] Write unit test `test_sweep_buy_multi_level_partial` in `engine/tests/order_book_test.rs` — pre-populate asks at [50000 qty 2, 52000 qty 3], submit `place_limit_buy("ORD-B6", 53000, 4)`, assert 2 trades: [(50000,2), (52000,2)] both taker_side=Buy, asks_changed=[PriceLevel{50000,0}, PriceLevel{52000,1}], no residual
- [x] T014 [P] [US3] Write unit test `test_sweep_buy_multi_level_residual` in `engine/tests/order_book_test.rs` — pre-populate asks at [50000 qty 1, 51000 qty 1, 52000 qty 1], submit `place_limit_buy("ORD-B7", 53000, 5)`, assert 3 trades consuming all asks, residual buy at 53000 qty 2 on bids per FR-009
- [x] T015 [P] [US3] Write unit test `test_sweep_sell_multi_level_residual` in `engine/tests/order_book_test.rs` — pre-populate bids at [51000 qty 2, 50000 qty 3], submit `place_limit_sell("ORD-S3", 49000, 6)`, assert 2 trades: [(51000,2), (50000,3)] both taker_side=Sell, all bids consumed, residual sell at 49000 qty 1 on asks per FR-009

**Checkpoint**: `cargo test` — 23 existing + 3 new US3 tests pass. Multi-level sweep verified for both sides.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final verification. Existing tests as regression guard, zero-allocation verification.

- [x] T016 Run `cargo test` — all tests pass (16 regression from 001/002 + 10 new crossing tests = 26 total)
- [x] T017 Verify zero-allocation: confirm `trades_buf` uses `Vec::with_capacity(128)`, crossing loop performs only in-place mutations and `Vec::remove(0)` shifts, single `clone()` for result construction, no `Box::new`, no heap in hot loop

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Verify baseline — can start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 — Trade struct, trades_buf, crossing loops in both methods
- **US1 (Phase 3)**: Tests for buy single-ask crossing — requires Phase 2 complete
- **US2 (Phase 4)**: Tests for partial fills + sell single-bid crossing — requires Phase 2 complete
- **US3 (Phase 5)**: Tests for multi-level sweep — requires Phase 2 complete
- **Polish (Phase 6)**: Final regression — requires all user story phases complete

### User Story Dependencies

- **US1 (P1)**: After Phase 2 — no other dependencies. Tests buy crossing (exact match, no-cross, price equality).
- **US2 (P2)**: After Phase 2 — independently testable. Tests partial fills and sell-side crossing (single level).
- **US3 (P3)**: After Phase 2 — independently testable. Tests multi-level sweep for both sides.

### Parallel Opportunities

- T002 (types.rs) can run in parallel with other Phase 2 tasks since it modifies a different file
- T006, T007, T008 in US1 can be written in parallel (all in same test file but different test functions)
- T009, T010, T011, T012 in US2 can be written in parallel
- T013, T014, T015 in US3 can be written in parallel
- US1, US2, US3 test phases can run in parallel after Phase 2 is complete

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Verify baseline
2. Complete Phase 2: Foundational (Trade struct + crossing loops in both methods)
3. Complete Phase 3: US1 tests (buy single-ask crossing)
4. **STOP and VALIDATE**: `cargo test` — all existing + US1 crossing tests pass. Buy-side matching is operational.

### Incremental Delivery

1. Baseline + Foundational → `cargo build` compiles with Trade struct and crossing logic
2. Add US1 tests → `cargo test` → MVP: buy crossing verified (3 new tests)
3. Add US2 tests → `cargo test` → partial fills + sell crossing verified (4 new tests)
4. Add US3 tests → `cargo test` → multi-level sweep verified (3 new tests)
5. Polish → full regression check, zero-allocation verification
