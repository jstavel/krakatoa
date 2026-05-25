(ns jstavel.zmq-transport.core
  (:import [org.zeromq ZMQ ZContext]))

(defn connect [addr]
  (let [context (ZContext.)
        socket (.createSocket context ZMQ/REQ)]
    (.connect socket addr)
    {:context context :socket socket}))

(defn send-message! [{:keys [socket]} ^bytes message]
  (.send socket message 0))

(defn receive-message! [{:keys [socket]}]
  (.recv socket 0))

(defn close! [{:keys [context socket]}]
  (.close socket)
  (.close context))
