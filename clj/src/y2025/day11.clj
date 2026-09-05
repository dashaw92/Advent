(ns y2025.day11
  (:require [clojure.string :as str]))

(defn line->node
  "Build a valid node mapping node-name: connection1 connection2 ... connectionN"
  [line]
  (let [[_ node rest] (re-matches #"(\w+): ([a-zA-Z ]+)" line)
        connections (re-seq #"\w+" rest)
        connections (map keyword connections)]
    {:node (keyword node) :conns connections}))

(defn lines [f]
  (as-> f $
        (slurp $)
        (str/split $ #"\n")
        (map line->node $)
        (reduce #(assoc %1 (:node %2) (:conns %2)) {} $)))

(defn terminal? [nodes node]
  (not (contains? nodes node)))

(defn routes [nodes start] :TODO)
;(defn routes [nodes start]
;  (reduce +
;          (letfn [(aux [nodes start route]
;                    (let [conns (nodes start)
;                          route (cons start route)
;                          all-routes
;                          (for [conn conns]
;                            (if (terminal? nodes conn)
;                              (cons conn route)
;                              (aux nodes conn route)))
;                          counts (mapv count all-routes)]
;                      counts))]
;            (aux nodes start []))))

(def example-input "src/y2025/d11e.txt")
(def example (lines example-input))
(def input-path "src/y2025/d11.txt")
(def input (lines input-path))

(routes input :you)