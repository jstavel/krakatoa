# Architectural Specification: Project Krakatoa

## 1. Introduction & Project Goal
Project Krakatoa is a high-performance, polyglot engineering portfolio (Proof of Work) designed for Senior Platform and QA Automation Engineering roles. The core objective is to demonstrate the architecture, implementation, and chaos-engineering verification of a distributed system designed for High Availability (HA) and extreme throughput (targeting 1M+ RPS). The project simulates a high-frequency cryptocurrency exchange matching engine with strict guarantees around state consistency and fault tolerance.

## 2. Global Architecture (Polyglot & Polylith Workspace)
The system pairs low-level, deterministic execution at the core with the structural safety, immutability, and modularity of functional component design for orchestration.

### Monorepo Directory Layout
krakatoa/
├── components/          # Pure business logic (shared, immutable, stateless)
│   ├── order-book-vld/  # Order book state validator (matches prices & executions)
│   ├── kafka-client/    # Abstraction for reading/writing the transaction log to Kafka
│   └── zmq-transport/   # Low-level ZeroMQ pipes for high-speed byte transfer
│
├── bases/               # Ingress gateways and exposed entry points (APIs / CLI)
│   ├── gateway-api/     # Client ingress layer (WebSockets / HTTP)
│   └── chaos-orchestra/ # Test execution engine and chaos orchestration controller
│
├── projects/            # Executable and deployable artifacts (Deployables)
│   ├── live-gateway/    # Composes the production Gateway (base + kafka + zmq)
│   └── tester-heavy/    # Composes the chaos simulation & load injection suite
│
├── engine/              # Decoupled subsystem: Core Matching Engine in Rust
└── infra/               # Decoupled subsystem: Packer / Terraform (KVM + Podman)

## 3. Component-Based Design (ZCA Principles)
The software architecture directly inherits principles of strict interface-implementation separation (reminiscent of the Zope Component Architecture - ZCA), adapted into a clean, functional paradigm:
* Strict Encapsulation: Every Clojure component exposes its public API exclusively via an interface.clj namespace. Internal state and implementation details are fully hidden.
* Reusability without Duplication: Components (e.g., validators or network transport abstractions) are shared verbatim between the production runtime (live-gateway) and the destructive test harness (tester-heavy), eliminating code duplication.
* REPL-Driven Development: The entire Polylith workspace can be spun up within a single Emacs CIDER REPL session, allowing for real-time, interactive debugging and execution of the entire distributed pipeline.

## 4. Technical Pillars & Optimizations

### Hybrid Infrastructure (infra)
* KVM (Kernel-based Virtual Machine): Managed via Terraform (libvirt) and Packer to generate minimal OS images (.qcow2). Each critical node (Kafka, Rust engine) runs inside its own VM with an isolated network stack to ensure exact network latency measurements.
* Podman: Drives rootless, daemonless execution of ancillary microservices (Prometheus, Grafana, Redis) without the memory overhead of spinning up complete virtual machines.

### High-Velocity Core Engine in Rust (engine)
* A low-level matching engine built with zero runtime memory allocation. It leverages techniques inspired by custom new constructor overloading, custom allocators, and memory pooling. It relies on the Rust Borrow Checker to guarantee thread safety and sub-millisecond, deterministic latency.

### Intelligent Hot-Failover Layer in Clojure
* Event Sourcing: Every inbound request is appended to Apache Kafka via the kafka-client component before downstream routing, ensuring 100% stable State Consistency and complete auditability.
* Lock-Free Routing: The IP address of the currently active Rust engine node is held inside a Clojure Atom. Resolving this address utilizes the hardware-level CAS (Compare-And-Swap) processor instruction. This allows millions of operations per second to stream asynchronously over ZeroMQ without thread contention (mutexes) or context switching.
* Hot Loading / Live Failover: Upon detecting a heartbeat loss from the primary Rust engine, the orchestration layer atomically swaps the address within the Atom. The Clojure gateway seamlessly routes subsequent packets to the backup node (Hot Standby). Because the backup node continuously consumes the same Kafka log in parallel, it maintains an identical in-memory order book and takes over as Master with zero message loss.

### High-Performance JVM Memory Management (1M+ RPS)
* Off-Heap Buffers: Utilizing direct byte buffers (DirectByteBuffers) via Netty/ZeroMQ. Clojure operates strictly at the pointer-shifting level without deserializing message payloads into high-level application objects inside the JVM heap.
* Ring Buffer (LMAX Disruptor Pattern): Employs a pre-allocated circular array structure for message indexing. Inbound data overwrites existing slots sequentially, eliminating runtime allocations and completely bypassing JVM Garbage Collector stop-the-world pauses. This concept natively implements object rotation with sentinel boundaries (Canary Values) to guard against memory corruption.

---

## 5. Project State & Execution Dashboard

### Current Status
* **Project Phase:** Initialization & Infrastructure Bootstrap
* **System State:** Greenfield (Empty repository, architecture finalized)
* **Last Modified:** 2026-05-24

### Active Slice: Milestone 1 — The Walking Skeleton
* **Goal:** Validate baseline end-to-end polyglot I/O. Establish a local, reactive pipeline from the Clojure gateway to the Rust matching engine over ZeroMQ, while concurrently persisting the raw transaction stream into a local Kafka broker.

#### Current Focus / Brain Anchor
* Initialize the root Monorepo directory structure, set up the bare Polylith workspace for Clojure, and scaffold an empty Rust crate in `engine/`. (This is the exact command-line starting point).

### Definition of Done (DoD) for Active Slice
* [ ] Local Kafka container runs smoothly via Podman Compose.
* [ ] Emacs CIDER REPL is successfully hooked into the Polylith workspace.
* [ ] Invoking a Clojure function sends raw bytes to the running Rust binary via ZeroMQ.
* [ ] Rust engine receives the payload, logs it, and replies with an `"OK"` byte acknowledgement.
* [ ] Clojure gateway records the incoming transaction to the local Kafka topic `order-log`.

## 6. Done Slices
* (No slices completed yet. Awaiting Milestone 1 execution.)
