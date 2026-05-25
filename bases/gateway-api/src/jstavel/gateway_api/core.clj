(ns jstavel.gateway-api.core
  (:require [jstavel.zmq-transport.interface :as zmq]
            [jstavel.kafka-client.interface :as kafka]))

(defn submit-order!
  "Orchestrates the order submission:
   1. Sends payload to Rust Engine via ZMQ.
   2. On ACK (0x06), persists to Kafka.
   Returns :ok or :error."
  [{:keys [zmq-socket kafka-producer topic payload]}]
  (zmq/send-message! zmq-socket payload)
  (let [reply (zmq/receive-message! zmq-socket)]
    (if (and reply (= (aget ^bytes reply 0) 0x06))
      (do
        (kafka/log-transaction! kafka-producer topic nil payload)
        :ok)
      :error)))

