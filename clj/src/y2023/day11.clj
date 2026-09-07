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

(defn expand-via-empty [expansion-factor lines]
  (loop [lines lines
         buffer []]
    (let [current-line (first lines)
          lines (rest lines)
          value (if (all-empty? current-line) expansion-factor 1)]
      (if (nil? current-line)
        buffer
        (recur lines (conj buffer value))))))

(defn expand-all [expansion-factor input]
  (let [expander (partial expand-via-empty expansion-factor)
        rows-expanded (expander (->rows input))
        cols-expanded (expander (->cols input))]
    [rows-expanded cols-expanded]))

(defn original-galaxy-coords [grid]
  (->> grid
       (map-indexed (fn [y line] (keep-indexed (fn [x item] (if (= \# item) [x y] nil)) line)))
       (apply concat)))

(defn galaxies [expansion-factor grid]
  (let [[ycoords xcoords] (expand-all expansion-factor grid)
        gs (original-galaxy-coords grid)]
    (for [[x y] gs
          :let [actual-x (reduce + (take x xcoords))
                actual-y (reduce + (take y ycoords))]]
      [actual-x actual-y])))

(defn dist [[x1 y1] [x2 y2]]
  (+ (abs (- x1 x2)) (abs (- y1 y2))))

(def example (read-input "src/y2023/d11e.txt"))
(def input (read-input "src/y2023/d11.txt"))

(defn all-pairs [gs]
  (apply concat (for [g1 (range (dec (count gs)))]
                  (for [g2 (range (inc g1) (count gs))]
                    [(nth gs g1) (nth gs g2)]))))

(defn solve [expansion-factor input]
  (let [gs (galaxies expansion-factor input)
        pairs (all-pairs gs)
        dists (map (partial apply dist) pairs)
        sum-dist (reduce + dists)]
    sum-dist))

(def p1-expansion 2)
(def p2-expansion (int 1e6))

(solve p1-expansion input)
(solve p2-expansion input)