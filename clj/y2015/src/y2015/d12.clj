(ns y2015.d12
  (:require [clojure.data.json :as json]))

(def doc (json/read-str (slurp "resources/day12.txt")))

(defn walk-doc [doc]
  (cond
    (instance? Number doc) doc
    (instance? clojure.lang.MapEntry doc) (let [[_ v] doc] (walk-doc v))
    (instance? java.util.Map doc) (reduce + (map walk-doc (vals doc)))
    (instance? String doc) 0
    :else (reduce + (map walk-doc doc))))

(Defn Walk-doc-p2 [doc]
  (cond
    (instance? Number doc) doc
    (instance? clojure.lang.MapEntry doc) (let [[_ v] doc] (walk-doc-p2 v))
    (instance? java.util.Map doc) (if (every? #(not= "red" %) (vals doc))
                                                     (reduce + (map walk-doc-p2 (vals doc)))
                                                     0)
    (instance? String doc) 0
    :else (reduce + (map walk-doc-p2 doc))))

(def part1 (walk-doc doc))
(def part2 (walk-doc-p2 doc))

(println "Part 1: " part1)
(println "Part 2: " part2)
