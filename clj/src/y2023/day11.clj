(ns y2023.day11
  (:require [clojure.string :as str]))

(defn read-input [f]
  (->> f
       slurp
       (str/split-lines)
       (map #(vec %))
       vec))

(defn all-empty? [line]
  (every? (partial = \.) line))

(defn ->rows [grid] grid)

(defn ->cols [grid]
  (let [len (count (first grid))]
    (for [y (range len)]
      (for [row (->rows grid)]
        (nth row y)))))

(defn empty-indices [grid dimension-extractor]
  (let [lines (dimension-extractor grid)
        empty-lines (map all-empty? lines)
        idx-pred (fn [idx bool-item] (if bool-item idx nil))
        idxs (keep-indexed idx-pred empty-lines)]
    idxs))

(defn expand-via-empty [lines empty]
  (loop [i 0
         empty empty
         lines lines
         buffer []]
    (let [current-line (first lines)
          lines (rest lines)]
      (if (nil? current-line)
        buffer
        (if (= i (first empty))
          (recur (inc i) (rest empty) lines (concat buffer [current-line current-line]))
          (recur (inc i) empty lines (concat buffer [current-line])))))))

(defn expand-all [input]
  (let [rows-expanded (expand-via-empty (->rows input) (empty-indices input ->rows))
        cols-expanded (expand-via-empty (->cols rows-expanded) (empty-indices rows-expanded ->cols))
        tranposed-to-normal (apply map vector cols-expanded)]
    tranposed-to-normal))

(def example (read-input "src/y2023/d11e.txt"))
;(def input (read-input "src/y2023/d11.txt"))
