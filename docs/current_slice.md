# Execution Plan: Milestone 1 — The Walking Skeleton

## Goal
Establish a reactive end-to-end pipeline: Clojure Gateway -> ZeroMQ -> Rust Engine -> Kafka Persistence.

## Step 1: Repository & Workspace Initialization
- [x] Initialize git repository.
- [x] Create the Polylith workspace structure using `clojure -M:poly create workspace name:krakatoa`.
- [x] Scaffold the component/base/project directories as per `specification.md`.
- [x] Initialize a new Rust project in `engine/` using `cargo init --bin`.

## Step 2: Infrastructure Bootstrap (Local)
- [ ] Create `infra/podman-local/compose.yaml` to spin up:
    - Redpanda (Kafka-compatible)
    - Zookeeper (if required by chosen Kafka distribution)
- [ ] Verify connectivity to Kafka using a local CLI tool.

## Step 3: ZeroMQ Component (Clojure)
- [ ] Create component `components/zmq-transport`.
- [ ] Add `interface.clj` defining `send-message!` and `receive-message!`.
- [ ] Implement using JZMQ or jeromq (Pure Java ZMQ implementation for REPL stability).

## Step 4: Rust Engine Skeleton
- [ ] Add `zmq` crate to `engine/Cargo.toml`.
- [ ] Implement a basic `REP` (Reply) socket listener in `engine/src/main.rs`.
- [ ] Logic: Receive bytes -> Log to stdout -> Return `[0x06]` (ACK).

## Step 5: Kafka Client Component (Clojure)
- [ ] Create component `components/kafka-client`.
- [ ] Add `interface.clj` defining `log-transaction!`.
- [ ] Implement using `clojure-kafka-client` or raw `kafka-clients` library.

## Step 6: The Integrated Base (Gateway API)
- [ ] Create base `bases/gateway-api`.
- [ ] Implement a function that:
    1. Accepts a payload.
    2. Sends it to the Rust Engine via `zmq-transport`.
    3. On `ACK`, asynchronously writes the payload to `kafka-client`.
- [ ] Create a `projects/live-gateway` to bundle these components.

## Step 7: Validation (REPL Driven)
- [ ] Start the Rust engine.
- [ ] Start the Polylith REPL (CIDER).
- [ ] Execute the gateway function from the REPL.
- [ ] Verify:
    - [ ] Rust logs the message.
    - [ ] Kafka topic `order-log` contains the message.
    - [ ] REPL receives the "OK" response.
