(ns jstavel.zmq-transport.interface
  (:require [jstavel.zmq-transport.core :as core]))

(defn connect [addr]
  (core/connect addr))

(defn send-message! [socket message]
  (core/send-message! socket message))

(defn receive-message! [socket]
  (core/receive-message! socket))

(defn close! [socket]
  (core/close! socket))
