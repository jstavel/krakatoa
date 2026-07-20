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

This will send an order through the entire pipeline: Clojure → ZeroMQ → Rust Engine → Kafka.

1.  In your CIDER REPL buffer, require the necessary namespaces:
    ```clojure
    (require '[jstavel.gateway-api.core :as gateway])
    (require '[jstavel.zmq-transport.interface :as zmq])
    (require '[jstavel.kafka-client.interface :as kafka])
    ```
    Evaluate each line (e.g., by placing the cursor after the closing parenthesis and pressing `C-x C-e`).

2.  Set up the components, send an order, and clean up:
    ```clojure
    (let [zmq-conn (zmq/connect "tcp://localhost:5555")       ;; ZMQ → Rust engine
          producer (kafka/create-producer "localhost:9092")    ;; Kafka producer
          topic    "order-log"
          payload  (.getBytes "ORDER-001,BUY,BTC,1.0,50000" "UTF-8")]
      (try
        (gateway/submit-order! {:zmq-socket     zmq-conn
                                :kafka-producer producer
                                :topic          topic
                                :payload        payload})
        (finally
          (zmq/close! zmq-conn)
          (kafka/close! producer))))
    ```
    Evaluate the entire `let` form. It should return `:ok` on success.

### 5. Verify the Results

Check the following to confirm the end-to-end pipeline is working:

1.  **Rust Engine Terminal:**
    *   Observe the terminal where your Rust engine is running (`cargo run`). You should see output indicating that it received the message from the Clojure gateway and sent back an ACK:
        ```
        Engine: Received 28 bytes
        ```

2.  **Kafka Topic (`order-log`):**
    *   Open a **new** terminal.
    *   Use the `rpk` CLI inside the Redpanda container:
        ```bash
        podman exec -it redpanda-krakatoa rpk topic consume order-log -f '%v\n'
        ```
    *   Alternatively, if `rpk` is available on your host:
        ```bash
        rpk topic consume order-log -f '%v\n' -b localhost:9092
        ```
    *   You should see the raw payload `ORDER-001,BUY,BTC,1.0,50000` printed, confirming the message was persisted to Kafka.

3.  **CIDER REPL Response:**
    *   The CIDER REPL should display `:ok` as the return value, confirming the full pipeline succeeded.

---

### 6. Final Verification Checklist

- [ ] **Rust Engine** logs `Engine: Received 28 bytes`
- [ ] **Kafka** topic `order-log` contains `ORDER-001,BUY,BTC,1.0,50000` (verified via `podman exec -it redpanda-krakatoa rpk topic consume order-log -f '%v\n'`)
- [ ] **CIDER REPL** returns `:ok`

Once all three checks pass, the end-to-end pipeline is confirmed to be operational.
