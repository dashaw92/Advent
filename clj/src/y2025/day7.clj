(ns y2025.day7
  (:require [clojure.string :as str]))

(defn lines [f]
  (-> f
      (slurp)
      (str/split #"\n")))

(def example-path "src/y2025/d7e.txt")
(def example (lines example-path))
(def input-path "src/y2025/d7.txt")
(def input (lines input-path))

(defn input->initial-beam-pos
  "Take the input and find the initial [X Y] position of the beam on the first line"
  [input]
  (-> input
      (first)
      (.indexOf "S")
      (cons [0])))

;(input->initial-beam-pos example)

(defn is-splitter?
  "Is the character at [x y] on the input grid a beam splitter?"
  [input [x y]]
  (-> input
      (nth y "")                                            ;Line y in the input
      (nth x \.)                                            ;Character x in the line
      (= \^)))                                              ;Splitters are denoted by '^' in the input
;(is-splitter? ["^..."] [10 0])

(defn split-beam
  "Convert a beam that hit a splitter into two new beams that are located diagonally down in both
  directions from the beam's previous location."
  [[x y]]
  [[(dec x) (inc y)]
   [(inc x) (inc y)]])

(defn move-beam-down
  "Increase the y component of the beam by 1 or split into two new beams if encountering a splitter"
  [splitter-fn? [x y]]
  (if (splitter-fn? [x (inc y)])
    (split-beam [x y])
    [[x (inc y)]]))

(defn solve-p1
  [input]
  (let [height (count input)
        is-splitter? (partial is-splitter? input)
        move-or-collide-beams #(move-beam-down is-splitter? %)]
    (loop [beams [(input->initial-beam-pos input)]
           splits 0]
      (if (every? #(>= (second %) height) beams)
        splits
        (let [previous-beam-count (count beams)
              next-beams (mapcat move-or-collide-beams beams)
              next-beam-count (count next-beams)
              beams (distinct next-beams)
              splits (+ splits (- next-beam-count previous-beam-count))]
          (recur beams splits))))))

(solve-p1 input)