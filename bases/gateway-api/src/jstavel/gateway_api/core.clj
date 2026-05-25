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

(defn -main [& _args]
  (println "Starting Gateway E2E Validation...")
  (let [zmq-conn (zmq/connect "tcp://localhost:5555")
        producer (kafka/create-producer "localhost:9092")
        topic "order-log"
        payload (.getBytes "ORDER-123,BUY,BTC,1.0,50000" "UTF-8")]
    (try
      (let [result (submit-order! {:zmq-socket zmq-conn
                                   :kafka-producer producer
                                   :topic topic
                                   :payload payload})]
        (println "Result:" result))
      (finally
        (zmq/close! zmq-conn)
        (kafka/close! producer)))))

