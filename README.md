# Krakatoa

High-performance polyglot matching engine (Clojure/Rust) built on Polylith architecture, featuring ZeroMQ transport, Kafka event sourcing, and chaos-testing.

## Overview

**Krakatoa** is a high-throughput, fault-tolerant distributed matching engine designed to demonstrate sub-millisecond hot-failover and 1M+ RPS capability on the JVM. The project serves as an advanced engineering portfolio (Proof of Work) focusing on systems architecture, low-level memory management, and automated resilient infrastructure.

By pairing a zero-allocation matching core written in **Rust** with a flexible, modular gateway and supervisor layer written in **Clojure**, Krakatoa achieves extreme performance without sacrificing architectural clarity.

## Project Kanban Board

| 📥 BACKLOG | ⏭️ NEXT | 🛠️ WIP | ✅ DONE |
| :--- | :--- | :--- | :--- |
| • [Milestone 3: HA/Chaos](kanban.org#milestone-3) | | • [Milestone 2: Engine](kanban.org#milestone-2) | • [Milestone 1: Skeleton](kanban.org#milestone-1) |
| • [Milestone 4: Frontend](kanban.org#milestone-4) | | | |

## Key Features & Pillars

- **Polylith Architecture:** The Clojure layer is built using strict component-based design, allowing 100% code reuse between production gateways and destruction testware.
- **Low-Latency Transport:** Communication across the polyglot boundary is driven by ZeroMQ utilizing off-heap direct byte buffers to minimize JVM Garbage Collection overhead.
- **State Consistency:** Full event-sourcing implementation backed by an Apache Kafka transaction log.
- **Automated Chaos Engineering:** The infrastructure is fully virtualized via KVM and managed via Terraform to rigorously test automated hot-failover state machines under simulated hardware drops.

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

The project follows a **Spec-Driven Development (SDD)** workflow powered by the [SpecKit](https://github.com/anomalyco/speckit) framework. Every feature goes through a structured pipeline: `specify → clarify → plan → tasks → implement → converge`. This ensures traceability from requirements to code and prevents scope drift.

**Primary references (normative):**

- [Constitution](.specify/memory/constitution.md) — Governing principles and non-negotiable constraints (v1.2.0)
- [Feature Specifications](specs/) — Per-feature specs, plans, task lists, data models, and contracts

**Secondary references (descriptive):**

- [Architectural Specification](docs/specification.md) — Global architecture, monorepo layout, technical pillars
- [Architecture Decision Records (ADRs)](docs/adr/) — Permanent records of major design decisions
- [Local Deployment Guide](docs/guides/local-stack-deployment.md) — How to run the full stack locally

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
