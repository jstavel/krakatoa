# Tasks: Limit Buy Order

**Input**: Design documents from `/specs/001-limit-buy/`

**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- Engine code: `engine/src/`
- Tests: `engine/tests/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the Rust module structure for the order book.

- [x] T001 Create `engine/src/types.rs` with module declaration — empty file, ready for type definitions
- [x] T002 Create `engine/src/order_book.rs` with module declaration — empty file, ready for implementation
- [x] T003 [P] Register `mod types` and `mod order_book` in `engine/src/main.rs` so they compile as part of the engine binary

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Data types that ALL user stories depend on. No user story work can begin until this phase is complete.

**⚠️ CRITICAL**: All types MUST derive `Debug, Clone, PartialEq` per spec for testability.

- [x] T004 [P] Define `Side` enum (Buy, Sell) in `engine/src/types.rs` with `#[derive(Debug, Clone, PartialEq)]`
- [x] T005 [P] Define `OrderStatus` enum (Accepted, Rejected) in `engine/src/types.rs` with `#[derive(Debug, Clone, PartialEq)]`
- [x] T006 [P] Define `PriceLevel` struct { price: u64, qty: u64 } in `engine/src/types.rs` with `#[derive(Debug, Clone, PartialEq)]`
- [x] T007 [P] Define `Trade` struct — placeholder, empty body, in `engine/src/types.rs` with `#[derive(Debug, Clone, PartialEq)]`
- [x] T008 [P] Define `BookSnapshot` struct { bids_changed: Vec\<PriceLevel\>, asks_changed: Vec\<PriceLevel\> } in `engine/src/types.rs` with `#[derive(Debug, Clone, PartialEq)]`
- [x] T009 Define `OrderResult` struct { order_id: String, status: OrderStatus, trades: Vec\<Trade\>, book_snap: BookSnapshot } in `engine/src/types.rs` with `#[derive(Debug, Clone, PartialEq)]` — depends on T005, T007, T008
- [x] T010 Implement `OrderBook::new()` in `engine/src/order_book.rs` — returns OrderBook with empty bids Vec initialized to a reasonable pre-allocated capacity per research.md (e.g., `Vec::with_capacity(128)`)

**Checkpoint**: Foundation ready — `cargo build` compiles. All types defined, OrderBook initializes empty.

---

## Phase 3: User Story 1 - Submit Limit Buy to Empty Order Book (Priority: P1) 🎯 MVP

**Goal**: Trader submits a limit buy to an empty book. Order is accepted, returned in bids_changed snapshot, no trades.

**Independent Test**: Create OrderBook::new(), call place_limit_buy("ORD-1", 50000, 1), assert OrderResult { Accepted, trades=[], bids_changed=[PriceLevel{50000, 1}] }

### Implementation for User Story 1

- [x] T011 [US1] Implement `OrderBook::place_limit_buy(order_id: String, price: u64, qty: u64) -> OrderResult` in `engine/src/order_book.rs` — search bids for matching price, if none found insert PriceLevel at correct descending-sorted position, return Accepted with bids_changed containing the new level. Use linear scan from research.md. Zero allocation on hot path per Constitution II.
- [x] T012 [US1] Write unit test `test_empty_book_insert` in `engine/tests/order_book_test.rs` — new book, insert ORD-1 at 50000 qty 1, assert Accepted, trades empty, bids_changed = [PriceLevel{50000, 1}]
- [x] T013 [US1] Write unit test `test_empty_book_two_different_prices` in `engine/tests/order_book_test.rs` — new book, insert ORD-1 at 50000, then ORD-2 at 51000, assert bids_changed order is [51000, 50000] descending

**Checkpoint**: `cargo test` — US1 tests pass. Empty book insert works. Sort order verified.

---

## Phase 4: User Story 2 - Submit Limit Buy to Non-Empty Order Book (Priority: P2)

**Goal**: Aggregation on same price, correct insertion between existing levels. Descending sort maintained.

**Independent Test**: Pre-populate book with limit buy at 50000 qty 1. Submit second at 50000 qty 2. Assert bids_changed shows one level at 50000 with qty 3.

### Implementation for User Story 2

- [x] T014 [US2] Extend `place_limit_buy` in `engine/src/order_book.rs` — when scanning bids and price matches existing level, aggregate qty (add to existing quantity) instead of inserting new level. Return bids_changed with the aggregated level.
- [x] T015 [US2] Write unit test `test_same_price_aggregation` in `engine/tests/order_book_test.rs` — insert 50000 qty 1, then insert 50000 qty 2, assert bids = [PriceLevel{50000, 3}]
- [x] T016 [US2] Write unit test `test_insert_between_levels` in `engine/tests/order_book_test.rs` — insert 50000 and 48000, then insert 49000, assert bids = [50000, 49000, 48000]

**Checkpoint**: `cargo test` — US1 + US2 tests pass. Aggregation works. Sort order survives insertions between levels.

---

## Phase 5: User Story 3 - Reject Invalid Limit Buy Orders (Priority: P3)

**Goal**: Invalid orders rejected without side effects. Book state unchanged.

**Independent Test**: Call place_limit_buy with price=0, assert Rejected, bids unchanged.

### Implementation for User Story 3

- [x] T017 [US3] Add validation guard at top of `place_limit_buy` in `engine/src/order_book.rs` — reject if price == 0 (return Rejected with empty bids_changed). Reject if qty == 0 (same). Reject if order_id empty string (same).
- [x] T018 [US3] Add duplicate order_id check in `place_limit_buy` in `engine/src/order_book.rs` — scan existing bids for the order_id (need to track per-level order_ids or maintain a set). Reject if already present.
- [x] T019 [US3] Write unit test `test_reject_price_zero` in `engine/tests/order_book_test.rs` — new book, place_limit_buy("ORD-1", 0, 1), assert Rejected, book bids empty
- [x] T020 [US3] Write unit test `test_reject_qty_zero` in `engine/tests/order_book_test.rs` — new book, place_limit_buy("ORD-1", 50000, 0), assert Rejected, book bids empty
- [x] T021 [US3] Write unit test `test_reject_duplicate_id` in `engine/tests/order_book_test.rs` — insert ORD-1 at 50000, then insert ORD-1 again at 51000, assert Rejected, book has only one bid
- [x] T022 [US3] Write unit test `test_reject_empty_id` in `engine/tests/order_book_test.rs` — place_limit_buy("", 50000, 1), assert Rejected

**Checkpoint**: `cargo test` — all US1, US2, US3 tests pass. Invalid orders never mutate book state.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Verify complete spec compliance.

- [x] T023 Run `cargo test` — all tests pass. Fix any failures.
- [x] T024 Verify zero-allocation: confirm no `Box::new`, no `String::new` (use `&str` or pre-allocated), no `Vec::push` (use with_capacity + within-capacity insert) on the hot path in `place_limit_buy`. Insert operations use Vec::insert which shifts elements but does NOT heap-allocate when within capacity.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **User Stories (Phase 3–5)**: All depend on Phase 2 completion
  - US2 naturally extends US1 implementation, but tests are independent
  - US3 is fully independent (validation guard at top of function)
- **Polish (Phase 6)**: Depends on all user stories being complete

### User Story Dependencies

- **US1 (P1)**: Can start after Phase 2 — no dependencies on other stories
- **US2 (P2)**: Extends US1 implementation (T014 touches same function), but tests are independent
- **US3 (P3)**: Can start after Phase 2 — validation guard is independent. T018 (duplicate check) may need order_id tracking added to the data model.

### Within Each User Story

- Implementation task before test tasks
- Tests can be written in any order within a story (all marked [P])
- Story complete before moving to next priority

### Parallel Opportunities

- T004, T005, T006, T007, T008 in Phase 2 can run in parallel (all types in same file but different structs — single write handles all)
- T012, T013 in US1 can be written in parallel after T011
- T019, T020, T021, T022 in US3 can be written in parallel after T017, T018

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: `cargo test` — US1 tests pass, demo via REP socket
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → `cargo build` compiles
2. Add US1 → `cargo test` → MVP: empty book works
3. Add US2 → `cargo test` → aggregation + sort verified
4. Add US3 → `cargo test` → all validation passes
5. Polish → final `cargo test`, zero-allocation audit
