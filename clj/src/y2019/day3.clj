(ns y2019.day3
  (:require [clojure.string :as str]
            [clojure.set]))

(defn parse-instr [s]
  (let [[_           dir      amt]
        (re-find #"(U|R|D|L)(\d+)" s)
        dir (keyword dir)
        amt (Integer/parseInt amt)]
    {:dir dir :amt amt}))

(defn to-instrs [line]
  (let [instrs (str/split line #",")
        instrs (map parse-instr instrs)]
    instrs))

(defn get-input [file]
  (as-> file $
      (slurp $)
      (str/split $ #"\n")
      (map to-instrs $)))

(defn delta
  "Generate the end position, axis, and step delta for any given instruction"
  [{:keys [dir amt]} [x y]]
  (case dir
        :U [x (- y amt 1) :y -1]
        :D [x (+ y amt 1) :y 1]
        :L [(- x amt 1) y :x -1]
        :R [(+ x amt 1) y :x 1]))

(defn run-instr
  "Generate all steps the current instr will take given the current (x, y)"
  [state instr [x y]]
  (let [[dx dy dir step] (delta instr [x y])
        cells (case dir
                :y (for [iy (range y dy step)] [x iy])
                :x (for [ix (range x dx step)] [ix y]))]
    [(into state cells) (last cells)]))

(defn reduce-wire
  "Wire run-instr into a reduce call"
  [[state lastPos] instr]
  (run-instr state instr lastPos))

(defn run-wire
  "Reduce the wire's instructions into [visited cells, last position]"
  [wire]
  (reduce reduce-wire [#{} [0 0]] wire))

(defn dist
  "Manhattan distance between (0, 0) and (x, y)"
  [[x y _]]
  (+ (abs (- 0 x)) (abs (- 0 y))))

(->> (get-input "src/y2019/d3.txt")
     (map run-wire)
     (map first)
     (reduce clojure.set/intersection)
     (filter #(not= % [0 0])) ;;origin doesn't count
     (map dist)
     (apply min))