(ns jstavel.kafka-client.interface
  (:require [jstavel.kafka-client.core :as core]))

(defn create-producer [bootstrap-servers]
  (core/create-producer bootstrap-servers))

(defn log-transaction! [producer topic key value]
  (core/log-transaction! producer topic key value))

(defn close! [producer]
  (core/close! producer))
