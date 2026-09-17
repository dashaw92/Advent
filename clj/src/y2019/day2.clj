(ns y2019.day2
  (:require [y2019.intcode :as intcode]))

(defn overwrite-cells
  "Overwrite cells at :cell x with :val y (via vec of maps)."
  [state cell->values]
  (loop [state state
         cell->values cell->values]
    (let [next (first cell->values)
          rest (rest cell->values)
          next-state (assoc-in state [:tape (:cell next)] (:val next))]
      (if (empty? rest)
        next-state
        (recur next-state rest)))))

(defn solve
  "This puzzle requires you to overwrite the values at cells 1 and 2 in the input
  tape to specific values. Pass the map of overwrites as a vec of maps with
  keys :cell and :val corresponding to the index and value to set. Afterwards,
  return the value of the cell at position 0 in the tape."
  [f overwrites]
  (let [init-state (intcode/read-state f)
        overwritten-state (overwrite-cells init-state overwrites)
        output-state (intcode/run overwritten-state)
        pos0 (first (:tape output-state))]
    pos0))

(def p1 (solve "src/y2019/d2.txt" [{:cell 1 :val 12} {:cell 2 :val 2}]))
(def p2
  (let [state (intcode/read-state "src/y2019/d2.txt")]
    (loop [noun 0
           verb 0]
      (let [modded-state (overwrite-cells state [{:cell 1 :val noun} {:cell 2 :val verb}])
            output (intcode/run modded-state)
            addr0 (first (:tape output))]
        (if (= 19690720 addr0)
          (+ (* noun 100) verb)
          (if (and (> noun 99) (> verb 99))
            (throw (IllegalStateException. "Exceeded limits for noun and verb (>99)."))
            (if (> noun 99)
              (recur 0 (inc verb))
              (recur (inc noun) verb))))))))

(println (str "Part 1: " p1 "\nPart 2: " p2))