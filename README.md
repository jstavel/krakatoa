# Krakatoa

High-performance polyglot matching engine (Clojure/Rust) built on Polylith architecture, featuring ZeroMQ transport, Kafka event sourcing, and chaos-testing.

## Overview

**Krakatoa** is a high-throughput, fault-tolerant distributed matching engine designed to demonstrate sub-millisecond hot-failover and 1M+ RPS capability on the JVM. The project serves as an advanced engineering portfolio (Proof of Work) focusing on systems architecture, low-level memory management, and automated resilient infrastructure.

By pairing a zero-allocation matching core written in **Rust** with a flexible, modular gateway and supervisor layer written in **Clojure**, Krakatoa achieves extreme performance without sacrificing architectural clarity.

## Key Features & Pillars

- **Polylith Architecture:** The Clojure layer is built using strict component-based design, allowing 100% code reuse between production gateways and destruction testware.
- **Low-Latency Transport:** Communication across the polyglot boundary is driven by ZeroMQ utilizing off-heap direct byte buffers to minimize JVM Garbage Collection overhead.
- **State Consistency:** Full event-sourcing implementation backed by an Apache Kafka transaction log.
- **Automated Chaos Engineering:** The infrastructure is fully virtualized via KVM and managed via Terraform to rigorously test automated hot-failover state machines under simulated hardware drops.

## Project Documentation

The repository follows a strict separation between long-term architectural specifications and temporary task management execution plans. You can explore the project depth via the following documentation paths:

- [Architectural Specification](docs/specification.md) — The living document outlining the global architecture, monorepo layout, technical pillars, and current project execution dashboard.
- [Architecture Decision Records (ADRs)](docs/adr/) — Dedicated directory containing permanent, immutable records of all major design decisions (e.g., component strategies, memory management patterns).
- [Active Slice Development Plan](docs/current_slice.md) — The granular, GTD-inspired task list currently being executed in the current development sprint.

## Monorepo Layout

```text
krakatoa/
├── components/          # Shared, immutable Clojure components (Polylith)
├── bases/               # Ingress gateways and exposed entry points
├── projects/            # Executable and deployable Clojure artifacts
├── engine/              # Core High-Velocity Matching Engine in Rust
├── infra/               # Infrastructure automation (Packer, Terraform, Podman)
└── docs/                # Project documentation, specifications, and ADRs
