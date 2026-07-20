<!-- SSOT: .specify/ + specs/ | This file: human-readable reference, NOT normative -->
# 3. Rust Engine Zero-Allocation Strategy

## Status
Proposed

## Context
Project Krakatoa targets a throughput of 1M+ RPS with sub-millisecond deterministic latency. In high-frequency trading systems, the primary source of latency jitter (tail latency) is non-deterministic memory management, such as Garbage Collection (GC) pauses in the JVM or heap fragmentation and allocation overhead in systems languages.

To achieve "The Walking Skeleton" and subsequent performance milestones, the Rust matching engine must minimize or eliminate heap allocations during the critical hot path of order processing.

## Decision
We will implement a zero-allocation architecture for the Rust matching engine using the following techniques:

1.  **Pre-allocated Memory Pools:** All data structures required for order processing (Order Book, Orders, Executions) will be stored in pre-allocated arrays or custom pool allocators initialized at startup.
2.  **Pointer-Based Messaging:** ZeroMQ integration will utilize `zmq::Message` buffers to move raw bytes directly into pre-allocated memory slots without intermediate object instantiation.
3.  **No-Standard-Library (where possible):** While we start with `std`, we will avoid types that perform implicit heap allocation (e.g., `String`, `Vec`, `Box`) in the core logic, preferring fixed-size arrays and slices.
4.  **Static Dispatch:** Use Generics and Traits to ensure compile-time polymorphism, avoiding the overhead of dynamic dispatch (trait objects).

## Consequences
*   **Complexity:** Development requires manual management of object lifecycles within pools and handling "pool full" scenarios.
*   **Safety:** We rely heavily on the Rust Borrow Checker to ensure that references to pooled objects do not outlive their validity.
*   **Performance:** Drastic reduction in tail latency (P99) and elimination of runtime allocation overhead.
*   **Predictability:** Memory usage becomes static and predictable at startup, simplifying infrastructure capacity planning.
