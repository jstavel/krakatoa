<!-- SSOT: .specify/ + specs/ | This file: human-readable reference, NOT normative -->
# 1. Use Polylith Architecture for Clojure Orchestrator

## Status
Approved

## Context
The Krakatoa orchestration and gateway layer needs to handle massive
concurrent scale while maintaining extreme internal modularity. We
need a way to share critical validation logic (like order book state
checking) and transport mechanisms between the production edge service
(`live-gateway`) and the destructive simulation suite (`tester-heavy`)
without creating code duplication or complex local dependency hell.

## Decision
We will implement the Clojure orchestrator using the **Polylith Architecture**. 
* Business logic will be decoupled into immutable, stateless **Components** (e.g., `order-book-vld`, `kafka-client`, `zmq-transport`).
* Public boundaries will be strictly enforced through `interface.clj` namespaces.
* Entry points will be managed via **Bases**, and deployable artifacts will be composed inside **Projects**.

## Consequences
* **Pros:** 100% code reusability across production and testing
  tools. Real-time REPL-Driven Development (via Emacs CIDER) across
  the entire monorepo workspace in a single session.
* **Cons:** Development team must adhere strictly to the Polylith tooling and workspace standards.
