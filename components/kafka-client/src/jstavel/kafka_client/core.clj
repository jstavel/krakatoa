(ns jstavel.kafka-client.core
  (:import [org.apache.kafka.clients.producer KafkaProducer ProducerRecord ProducerConfig]
           [java.util Properties]))

(defn create-producer [bootstrap-servers]
  (let [props (Properties.)]
    (.put props ProducerConfig/BOOTSTRAP_SERVERS_CONFIG bootstrap-servers)
    (.put props ProducerConfig/KEY_SERIALIZER_CLASS_CONFIG "org.apache.kafka.common.serialization.StringSerializer")
    (.put props ProducerConfig/VALUE_SERIALIZER_CLASS_CONFIG "org.apache.kafka.common.serialization.ByteArraySerializer")
    (.put props ProducerConfig/ACKS_CONFIG "all")
    (KafkaProducer. props)))

(defn log-transaction! [^KafkaProducer producer topic key ^bytes value]
  (let [record (ProducerRecord. topic key value)]
    (.send producer record)))

(defn close! [^KafkaProducer producer]
  (.close producer))
