(ns y2025.day9
  (:require [clojure.string :as str]))

(defn extract-coords
  "Extract x,y coordinate pair from input line"
  [line]
  (let [[_ x y] (re-matches #"(\d+),(\d+)" line)
        x (Long/parseLong x)
        y (Long/parseLong y)]
  [x y]))

(defn lines [f]
  (as-> f $
      (slurp $)
      (str/split $ #"\n")
      (map extract-coords $)))

(defn rect->area
  "Given two [x,y] coordinates, calculate the area of the rectangle the coordinates form"
  [r1 r2]
  ;find the difference between the given [x y] points, get the absolute value, and then increment each (the area is inclusive)
  ;reduce the found differences via * to get area
  (reduce * (map (comp inc abs -) r1 r2)))

(def example-path "src/y2025/d9e.txt")
(def example (lines example-path))

(def input-path "src/y2025/d9.txt")
(def input (lines input-path))

(defn solve-p1
  "Find the largest possible rectangle given all coordinates"
  [input]
  (apply max (for [r1 input
                   r2 input
      :when (not= r1 r2)]
  (rect->area r1 r2))))

(solve-p1 example) ;50
(solve-p1 input)