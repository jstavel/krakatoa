# AGENTS: Krakatoa Documentation Hierarchy

## Single Source of Truth (SSOT)

These sources are **normative** — when implementing or planning, read from here first:

```
specs/NNN-feature-name/spec.md    ← WHAT to build (feature spec)
specs/NNN-feature-name/plan.md    ← HOW to build it (implementation plan)
specs/NNN-feature-name/tasks.md   ← Execution breakdown
.specify/memory/constitution.md   ← PRINCIPLES governing all features
.specify/feature.json             ← Active feature context
```

## SpecKit Workflow

```
/speckit.specify  → creates spec.md, validates quality
      ↓ gate: review
/speckit.plan     → creates plan.md, research.md, data-model.md, contracts/
      ↓ gate: review
/speckit.tasks    → creates tasks.md (Setup → Foundational → US1 → US2 → US3 → Polish)
/speckit.implement → writes the actual code
```

## Secondary Sources (human-readable, NOT normative)

These files serve **people** for context, understanding, and historical reference.
They are NOT authoritative for implementation decisions:

```
docs/specification.md   ← Project overview, architectural narrative (stale in parts)
docs/adr/               ← Historical decision records (WHY a choice was made)
docs/current_slice.md   ← Archived execution plan (M1 complete)
kanban.org              ← GTD board, backlog, migrated items
```

## When in doubt

The constitution (`.specify/memory/constitution.md`) is the highest authority.
If any secondary source contradicts `.specify/` or `specs/`, the SpecKit sources win.

## Contract Pattern (Constitution: Observable Operations)

Every operation contract MUST enable invoke↔complete pairing.
Use this pattern in `contracts/` via Malli schemas in markdown code blocks:

```clojure
;; INVOKE:  place_limit_buy(order_id, price, qty)
;; COMPLETE: OrderResult { order_id, status, trades, book_snap }

(def OrderResult
  [:map
   [:order-id  :string]    ;; echo — enables invoke↔complete pairing
   [:status    OrderStatus]
   [:trades    [:vector Trade]]
   [:book-snap BookSnapshot]])
```

The echoed identifier is the minimum requirement; gateways and observers
(Kafka, Jepsen) use it to correlate requests with results without the
engine performing any logging or allocation on the hot path.
