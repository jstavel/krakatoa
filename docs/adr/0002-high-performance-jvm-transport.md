# 2. High-Performance JVM Transport & Memory Management

## Status
Approved

## Context
The Clojure gateway functions as a high-speed routing switch between
clients and the native Rust matching engine. To process over 1,000,000
requests per second (1M+ RPS) without introducing unacceptable latency
spikes, the JVM layer cannot afford standard object allocation
overhead, which triggers heavy Garbage Collection (GC) stop-the-world
pauses.

## Decision
We will employ a combination of nízkoúrovňových (low-level) memory management patterns directly on the JVM:
1. **Off-Heap Buffers:** Use ZeroMQ/Netty `DirectByteBuffers` to
   stream raw bytes without parsing payloads into high-level Clojure
   data structures at the ingress boundary (Pointer Shifting).
2. **Ring Buffer Pattern:** Implement a pre-allocated circular array
   (inspired by the LMAX Disruptor pattern) for indexing internal
   events. Incoming data will overwrite existing slots sequentially,
   guaranteeing zero runtime allocation during hot paths.

## Consequences
* **Pros:** Deterministic, sub-millisecond latencies on the
  JVM. Complete elimination of Garbage Collector spikes during peak
  load.
* **Cons:** Increased code complexity. Manual tracking of memory
  layouts and slot reuse boundaries, requiring strict sentinel
  (Canary) checking to guard against memory corruption.
