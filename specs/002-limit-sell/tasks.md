# Tasks: Limit Sell Order

**Input**: Design documents from `/specs/002-limit-sell/`

**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Organization**: Tasks grouped by user story. Extends existing code — modifies `engine/src/order_book.rs` and `engine/tests/order_book_test.rs`.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- Engine code: `engine/src/`
- Tests: `engine/tests/`

---

## Phase 1: Setup

**Purpose**: No new files needed — feature extends existing OrderBook.

- [x] T001 Verify `cargo test` passes with all 8 existing tests before modification

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Add asks side to OrderBook. Types already exist from 001.

- [x] T002 Add `asks: Vec<PriceLevel>` field to `OrderBook` struct in `engine/src/order_book.rs` — initialize with `Vec::with_capacity(128)` in `new()`
- [x] T003 Add `place_limit_sell` method signature to `impl OrderBook` in `engine/src/order_book.rs` — `pub fn place_limit_sell(&mut self, order_id: String, price: u64, qty: u64) -> OrderResult`

**Checkpoint**: `cargo build` compiles. OrderBook has both bids and asks fields. Existing 8 tests still pass.

---

## Phase 3: User Story 1 - Submit Limit Sell to Empty Order Book (Priority: P1) 🎯 MVP

**Goal**: Trader submits a limit sell to an empty book. Order accepted, placed on asks side with ascending sort.

**Independent Test**: Create empty OrderBook, call place_limit_sell("ORD-1", 50000, 1), assert Accepted, trades=[], asks_changed=[PriceLevel{50000, 1}]

### Implementation for User Story 1

- [x] T004 [US1] Implement `place_limit_sell` body in `engine/src/order_book.rs` — linear scan asks for insertion position using `position(\|l\| l.price > price)` pattern from research.md (ascending sort). Create PriceLevel, clone for book insert, return OrderResult with Accepted and asks_changed. Reuse order_ids check at top. Zero allocation per research.md.
- [x] T005 [US1] Write unit test `test_empty_book_sell_insert` in `engine/tests/order_book_test.rs` — new book, insert ORD-1 at 50000 qty 1, assert Accepted, trades empty, asks_changed = [PriceLevel{50000, 1}], bids_changed empty
- [x] T006 [US1] Write unit test `test_empty_book_sell_two_prices` in `engine/tests/order_book_test.rs` — insert 50000 then 52000, assert asks_changed shows correct ascending order [50000, 52000]

**Checkpoint**: `cargo test` — 8 existing + 2 new tests pass.

---

## Phase 4: User Story 2 - Submit Limit Sell to Non-Empty Order Book (Priority: P2)

**Goal**: Aggregation on same price, correct ascending insertion between levels.

**Independent Test**: Pre-populate asks with 50000 qty 1. Submit second at 50000 qty 2. Assert asks_changed shows 50000 with qty 3.

### Implementation for User Story 2

- [x] T007 [US2] Add aggregation logic to `place_limit_sell` in `engine/src/order_book.rs` — when scanning asks and price matches existing level, aggregate qty. Return asks_changed with updated quantity.
- [x] T008 [US2] Write unit test `test_sell_same_price_aggregation` in `engine/tests/order_book_test.rs` — insert 50000 qty 1, insert 50000 qty 2, assert asks_changed = [PriceLevel{50000, 3}]
- [x] T009 [US2] Write unit test `test_sell_insert_between_levels` in `engine/tests/order_book_test.rs` — insert 50000 and 52000, insert 51000, assert asks = [50000, 51000, 52000] ascending

**Checkpoint**: `cargo test` — 10 existing + 2 new tests pass.

---

## Phase 5: User Story 3 - Reject Invalid Limit Sell Orders (Priority: P3)

**Goal**: Invalid sell orders rejected. Cross-side ID uniqueness enforced.

**Independent Test**: Call place_limit_sell with price=0, assert Rejected, asks unchanged.

### Implementation for User Story 3

- [x] T010 [US3] Add validation guard at top of `place_limit_sell` in `engine/src/order_book.rs` — reject if price == 0, qty == 0, or order_id is empty. Returns Rejected with empty BookSnapshot. Reject if order_id already in `self.order_ids` (cross-side check per FR-007).
- [x] T011 [US3] Write unit test `test_sell_reject_price_zero` in `engine/tests/order_book_test.rs` — place_limit_sell price=0, assert Rejected, asks unchanged
- [x] T012 [US3] Write unit test `test_sell_reject_qty_zero` in `engine/tests/order_book_test.rs` — place_limit_sell qty=0, assert Rejected
- [x] T013 [US3] Write unit test `test_sell_reject_duplicate_id` in `engine/tests/order_book_test.rs` — insert ORD-1 as buy, then place_limit_sell ORD-1, assert Rejected (cross-side check)
- [x] T014 [US3] Write unit test `test_sell_reject_empty_id` in `engine/tests/order_book_test.rs` — place_limit_sell empty string, assert Rejected

**Checkpoint**: `cargo test` — 12 existing + 4 new tests pass. Cross-side uniqueness verified.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Verify all existing and new tests pass.

- [x] T015 Run `cargo test` — all tests pass (8 existing buy + 8 new sell)
- [x] T016 Verify zero-allocation: asks uses same Vec::with_capacity strategy, no Box::new, no heap on hot path

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Verify baseline — can start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 — adds asks field, method signature
- **US1 (Phase 3)**: Depends on Phase 2
- **US2 (Phase 4)**: Extends US1 implementation (same function)
- **US3 (Phase 5)**: Depends on Phase 2 (validation guard is independent of aggregation)
- **Polish (Phase 6)**: Depends on all user stories complete

### User Story Dependencies

- **US1 (P1)**: After Phase 2 — no other dependencies
- **US2 (P2)**: Extends US1 — touches same function but tests independent
- **US3 (P3)**: After Phase 2 — validation guard + cross-side check independent of sort/aggregation

### Parallel Opportunities

- T005, T006 in US1 can be written in parallel after T004
- T008, T009 in US2 can be written in parallel after T007
- T011, T012, T013, T014 in US3 can be written in parallel after T010

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Verify baseline
2. Complete Phase 2: Foundational (asks field + method signature)
3. Complete Phase 3: US1
4. **STOP and VALIDATE**: `cargo test` — all existing + US1 sell tests pass

### Incremental Delivery

1. Baseline + Foundational → `cargo build` compiles with asks field
2. Add US1 → `cargo test` → MVP: empty asks works
3. Add US2 → `cargo test` → aggregation + ascending sort verified
4. Add US3 → `cargo test` → validation + cross-side uniqueness
5. Polish → final verification
