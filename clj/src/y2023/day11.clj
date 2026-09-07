(ns y2023.day11
  (:require [clojure.string :as str]))

(defn read-input [f]
  (->> f
       slurp
       (str/split-lines)
       (map #(vec %))
       vec))

(defn all-empty? [line] (every? (partial = \.) line))

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

(defn galaxies [grid]
  (->> grid
       expand-all
       (map-indexed (fn [y line] (keep-indexed (fn [x item] (if (= \# item) [x y] nil)) line)))
       (apply concat)))

(defn dist [[x1 y1] [x2 y2]]
  (+ (abs (- x1 x2)) (abs (- y1 y2))))

(def example (read-input "src/y2023/d11e.txt"))
;(def input (read-input "src/y2023/d11.txt"))

(defn all-pairs [gs]
  (apply concat (for [g1 (range (dec (count gs)))]
                  (for [g2 (range (inc g1) (count gs))]
                    [(nth gs g1) (nth gs g2)]))))

(reduce + (map (partial apply dist) (all-pairs (galaxies example))))