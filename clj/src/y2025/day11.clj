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

(def example-input "src/y2025/d11e.txt")
(def example (lines example-input))


