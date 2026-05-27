# How to Run the Krakatoa Stack Locally

This document provides step-by-step instructions to bring up and test the entire Krakatoa stack (Clojure Gateway -> ZeroMQ -> Rust Engine -> Kafka Persistence) locally using Podman (or Docker) and Emacs with CIDER.

## Prerequisites

*   **Podman** (or Docker) installed and running.
*   **Clojure CLI tools** installed.
*   **Rust toolchain** installed.
*   **Emacs** with **CIDER** installed and configured for Clojure development.
*   Ensure your `projects/live-gateway/deps.edn` (or root `deps.edn` with a suitable alias) has `nrepl` and `cider-nrepl` configured for CIDER.

---

## Step-by-Step Instructions

### 1. Start Infrastructure (Redpanda/Kafka)

This will spin up a Kafka-compatible Redpanda cluster using `podman-compose`.

1.  Open a terminal.
2.  Navigate to the `infra/podman-local/` directory:
    ```bash
    cd infra/podman-local/
    ```
3.  Start the services in detached mode:
    ```bash
    podman-compose up -d
    ```
4.  Verify Redpanda containers are running:
    ```bash
    podman-compose ps
    ```

### 2. Start the Rust Engine

This will compile and run your Rust engine, starting its ZeroMQ `REP` (Reply) socket listener.

1.  Open a **new** terminal window (keep it open to observe logs).
2.  Navigate to the `engine/` directory:
    ```bash
    cd engine/
    ```
3.  Run the Rust engine:
    ```bash
    cargo run
    ```
    You should see output indicating it's listening, for example: `Listening on tcp://127.0.0.1:5555`.

### 3. Connect to a Clojure REPL (CIDER)

This connects your Emacs CIDER to the `live-gateway` project.

1.  Open a **new** terminal window.
2.  Navigate to the `projects/live-gateway/` directory:
    ```bash
    cd projects/live-gateway
    ```
    This ensures CIDER loads the correct project context and dependencies.
3.  In Emacs, type `M-x cider-jack-in-clj` (or `C-c M-j`).
4.  When prompted, select `clj`.
5.  If CIDER asks to select a `deps.edn` alias, choose the alias configured for development and CIDER (e.g., `:dev` or a custom alias like `:cider`).
    CIDER will start an nREPL server and connect to it. You should see a new `*cider-repl <project-name>*` buffer appear.

### 4. Execute the Gateway Function

This will send an order through the entire pipeline.

1.  In your CIDER REPL buffer, type the following Clojure code to require the `gateway-api.core` namespace:
    ```clojure
    (require '[jstavel.gateway-api.core :as gateway])
    ```
    Evaluate this line (e.g., by placing the cursor after the closing parenthesis and pressing `C-x C-e`).
2.  Now, call the `submit-order!` function with a sample payload. The `localhost:9092` should match your Redpanda's Kafka listener address.
    ```clojure
    (gateway/submit-order! "localhost:9092" {:order-id "ORD-001" :price 100 :quantity 10})
    ```
    Evaluate this form.

### 5. Verify the Results

Check the following to confirm the end-to-end pipeline is working:

1.  **Rust Engine Terminal:**
    *   Observe the terminal where your Rust engine is running (`cargo run`). You should see output indicating that it received the message from the Clojure gateway and sent back an ACK.
2.  **Kafka Topic (`order-log`):**
    *   Open a **new** terminal.
    *   Use the `rpk` CLI (part of Redpanda, if installed and in your PATH) to consume messages from the `order-log` topic:
        ```bash
        rpk topic consume order-log -f '%v\n'
        ```
    *   Alternatively, if you have standard Kafka CLI tools installed:
        ```bash
        kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic order-log --from-beginning
        ```
    *   You should see the payload `{"order-id" "ORD-001" :price 100 :quantity 10}` (or a similar serialized form of your order map) printed, confirming the message was persisted to Kafka.
3.  **CIDER REPL Response:**
    *   The CIDER REPL should display the return value of the `(gateway/submit-order! ...)` call. This should indicate success (e.g., `nil`, `:ok`, a map with status, etc.), depending on your `submit-order!` implementation.

---

Once all these checks pass, the end-to-end pipeline is confirmed to be operational.
