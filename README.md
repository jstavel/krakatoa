# Krakatoa

Early-stage polyglot matching engine (Rust/Clojure) on Polylith architecture.
The Rust matching core runs and is tested; the Clojure transport/supervisor layer
is scaffolded, and the chaos/Jepsen-style verification layer is specified but not
yet built.

## Overview

**Krakatoa** is an in-progress distributed matching engine, built in public as a
proof of work in systems architecture and spec-first delivery. The **target**
design is sub-millisecond hot-failover with event sourcing over Kafka — those are
targets on the roadmap, not measured results.

What exists today is the part that can be checked: a **Rust matching core** with a
limit order book, price–time priority, taker–maker crossing, partial fills and
multi-level sweeps, covered by 26 `cargo test` cases across three spec-first specs.
**Zero-allocation on the hot path** is a design constraint of that core, not yet
a benchmarked claim.

## Project Kanban Board

| 📥 BACKLOG | ⏭️ NEXT | 🛠️ WIP | ✅ DONE |
| :--- | :--- | :--- | :--- |
| • [Milestone 3: HA/Chaos](kanban.org#milestone-3) | | • [Milestone 2: Engine](kanban.org#milestone-2) | • [Milestone 1: Skeleton](kanban.org#milestone-1) |
| • [Milestone 4: Frontend](kanban.org#milestone-4) | | | |

## Status — honest by design

| Area | State |
|---|---|
| Rust matching core (limit buy/sell, crossing, partial fills, sweeps) | **Built & tested** — 26 `cargo test` cases, specs 001–003 |
| Market orders, cancellation, memory pool | **Specified** (spec 004) — not implemented |
| Clojure/Polylith components: ZeroMQ transport, Kafka client, chaos orchestrator | **Scaffolded** — components, interfaces and specs exist; stubs, not working transport |
| Infra | **Local compose only** (`infra/podman-local`) — no KVM/Terraform stack yet |
| Chaos / distributed consistency verification (Jepsen-style) | **Planned** — Milestone 3, backlog |
| Frontend (re-frame, Wyckoff-inspired tape) | **Vision only** |

## Design pillars (target architecture)

- **Polylith Architecture:** strict component-based design for the Clojure layer, so
  production gateways and testware can share code.
- **Low-Latency Transport (planned):** ZeroMQ across the polyglot boundary, off-heap
  direct byte buffers to reduce GC pressure — the component is scaffolded.
- **State Consistency (planned):** event sourcing backed by a Kafka transaction log —
  the client component is scaffolded; no working pipeline yet.
- **Chaos Engineering (planned):** KVM- and Terraform-managed environments to test
  hot-failover state machines under simulated hardware drops. This is the reason the
  project exists; it is not implemented, so no chaos results are claimed.

## Engine Status (Milestone 2 — In Progress)

The Rust matching engine implements a limit order book with active crossing:

| Feature | Spec | Tests | Status |
|---|---|---|---|
| 001 — Limit Buy Order | [spec.md](specs/001-limit-buy/spec.md) | 8 | ✅ Done |
| 002 — Limit Sell Order | [spec.md](specs/002-limit-sell/spec.md) | 8 | ✅ Done |
| 003 — Order Matching (Crossing) | [spec.md](specs/003-order-matching/spec.md) | 10 | ✅ Done |
| **Total** | | **26** | `cargo test` passes |

**Capabilities**: Limit buy/sell orders with price-time priority, taker-maker crossing, partial fills, multi-level sweeps, and Trade record generation — all zero-allocation on the hot path.

**Upcoming**: Market orders, order cancellation, memory pool/allocator.

## Project Documentation

The project follows a **Spec-Driven Development (SDD)** workflow powered by the [SpecKit](https://github.github.com/spec-kit/) framework. Every feature goes through a structured pipeline: `specify → clarify → plan → tasks → implement → converge`. This ensures traceability from requirements to code and prevents scope drift.

**Primary references (normative):**

- [Constitution](.specify/memory/constitution.md) — Governing principles and non-negotiable constraints (v1.2.0)
- [Feature Specifications](specs/) — Per-feature specs, plans, task lists, data models, and contracts

**Secondary references (descriptive):**

- [Architectural Specification](docs/specification.md) — Global architecture, monorepo layout, technical pillars
- [Vision](docs/vision.md) — The full loop: engine → analysis → re-frame UI, Wyckoff-inspired trade tape
- [Architecture Decision Records (ADRs)](docs/adr/) — Permanent records of major design decisions
- [Local Deployment Guide](docs/guides/local-stack-deployment.md) — How to run the full stack locally
- [RFC: kraken-tap](docs/rfc-kraken-tap.md) — Market data capture & replay tool (draft for discussion)

## Monorepo Layout

```text
krakatoa/
├── components/         # Shared, immutable Clojure components (Polylith)
├── bases/              # Ingress gateways and exposed entry points
├── projects/           # Executable and deployable Clojure artifacts
├── engine/             # Core High-Velocity Matching Engine in Rust
├── infra/              # Infrastructure automation (Packer, Terraform, Podman)
└── docs/               # Project documentation, specifications, and ADRs
```
