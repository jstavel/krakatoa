<!--
Sync Impact Report
Version change: 1.1.0 → 1.2.0
Modified principles: Development Workflow — added "Observable Operations" rule
Added sections: None
Removed sections: None
Templates requiring updates:
  - .specify/templates/plan-template.md ✅ (no change needed)
  - .specify/templates/spec-template.md ✅ (no change needed)
  - .specify/templates/tasks-template.md ✅ (no change needed)
Follow-up TODOs:
  - .opencode/AGENTS.md ⚠ — add invoke↔complete contract pattern example
-->

# Krakatoa Constitution

## Core Principles

### I. Polylith Component Purity

Every Clojure component MUST expose its public API exclusively via an
`interface.clj` namespace. Internal state and implementation details
MUST remain fully hidden. Components MUST be shared verbatim between
the production runtime (`live-gateway`) and the destructive test harness
(`tester-heavy`), eliminating code duplication and ensuring behavioral
consistency across all environments.

**Rationale**: Strict interface-implementation separation enables 100%
code reuse between production and testware, eliminating an entire class
of "it works in tests" bugs and ensuring chaos tests exercise the exact
same code paths as production traffic.

### II. Zero-Allocation Performance

The Rust matching engine MUST operate with zero runtime memory allocation.
All Clojure gateway operations MUST utilize off-heap direct byte buffers
via ZeroMQ, avoiding JVM heap allocation entirely. Message payloads MUST
NOT be deserialized into high-level application objects inside the JVM
heap during hot-path processing.

**Rationale**: Sub-millisecond latency and 1M+ RPS throughput require
eliminating garbage collection pauses. Zero-allocation at the engine
level and off-heap buffers at the gateway level are non-negotiable for
meeting performance targets.

### III. Event Sourcing First

Every inbound request MUST be appended to Apache Kafka via the
`kafka-client` component BEFORE any downstream routing occurs. State
consistency MUST be derived exclusively from the Kafka transaction log.
No component MAY maintain mutable state that cannot be reconstructed
from the event log.

**Rationale**: Event sourcing ensures 100% state consistency and complete
auditability. It enables hot failover by allowing standby nodes to
reconstruct identical in-memory state from the shared log, guaranteeing
zero message loss during failover events.

### IV. Chaos-Driven Verification

Automated chaos engineering MUST be an integral part of the development
lifecycle. The infrastructure MUST be fully virtualized via KVM and
managed via Terraform to enable rigorous testing of automated hot-failover
state machines under simulated hardware failures. Every fault-tolerance
claim MUST be empirically verified through controlled chaos experiments.

**Rationale**: Theoretical fault tolerance is insufficient. Only automated,
reproducible chaos experiments can provide high-confidence evidence that
the system behaves correctly under real-world failure conditions.

### V. Polyglot Boundary Excellence

Cross-language communication (Clojure ↔ Rust) MUST utilize ZeroMQ with
off-heap direct byte buffers. The hot-path IP address of the active
Rust engine node MUST be held in a Clojure Atom and resolved via
hardware-level CAS (Compare-And-Swap) for lock-free routing. Failover
MUST be atomic and seamless, with no thread contention or context switching.

**Rationale**: The polyglot boundary is the system's most sensitive
performance bottleneck. Lock-free routing via CAS and ZeroMQ transport
enable millions of operations per second without mutex contention or
GC-induced latency spikes.

## Technical Constraints

- **KVM Isolation**: Critical nodes (Kafka, Rust engine) MUST run inside
  individual VMs with isolated network stacks for precise latency measurement.
- **Rootless Containers**: Ancillary microservices (Prometheus, Grafana,
  Redis) MUST run via Podman without daemon overhead.
- **Ring Buffer Pattern**: The gateway MUST employ pre-allocated circular
  array structures (LMAX Disruptor pattern) with sentinel boundaries for
  message indexing.
- **REPL-Driven Development**: The entire Polylith workspace MUST be
  callable from a single Emacs CIDER REPL session for interactive debugging.

## Development Workflow

- **Component Isolation**: New components MUST be developed and tested in
  isolation before integration into the Polylith workspace.
- **Contract First**: Cross-component interfaces MUST be defined as Malli
  schemas in markdown code blocks within `contracts/` before implementation
  begins. Contracts serve as both human-readable documentation and
  machine-verifiable specifications for the SpecKit workflow.
- **Chaos Testing Gate**: No feature MAY be merged without corresponding
  chaos test coverage for fault-tolerance scenarios.
- **Observable Operations**: Every state-mutating operation MUST produce
  a traceable record enabling observers to correlate requests with results.
- **REPL Validation**: All gateway code MUST be validated interactively
  via CIDER REPL before committing.

## Governance

This constitution supersedes all other development practices for the
Krakatoa project. Amendments require:

1. Documentation of the proposed change with rationale
2. Impact analysis on existing components and tests
3. Version bump following semantic versioning rules
4. Update of all dependent templates and documentation

All pull requests and code reviews MUST verify compliance with these
principles. Complexity MUST be justified against the simplest adequate
solution.

**Version**: 1.2.0 | **Ratified**: 2026-07-16 | **Last Amended**: 2026-07-16
