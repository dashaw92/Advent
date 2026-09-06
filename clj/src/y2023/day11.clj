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

(defn expand-via-empty [lines]
  (loop [lines lines
         buffer []]
    (let [current-line (first lines)
          lines (rest lines)]
      (if (nil? current-line)
        buffer
        (if (all-empty? current-line)
          (recur lines (concat buffer (repeat 2 current-line)))
          (recur lines (concat buffer [current-line])))))))

(defn expand-all [input]
  (let [rows-expanded (expand-via-empty (->rows input))
        cols-expanded (expand-via-empty (->cols rows-expanded))
        transposed-to-normal (apply map vector cols-expanded)]
    transposed-to-normal))

(def example (read-input "src/y2023/d11e.txt"))
;(def input (read-input "src/y2023/d11.txt"))