(ns y2019.day5
  (:require [y2019.intcode :as intcode]))

(intcode/run (intcode/lit->state "1002,4,3,4,33" [0]))                      ;correctly halts {:tape [1002 4 3 4 99] :halted true ..}
(def p1
  (let [state (intcode/run (intcode/read-state "src/y2019/d5.txt" [1]))
        all-passed (every? zero? (rest (:output state)))
        output0 (first (:output state))]
    (if all-passed
      (println (str "Passed all diagnostic tests. Part 1 is " output0))
      (println "Did not pass diagnostic tests."))
    output0))
(def p2
  (let [state (intcode/run (intcode/read-state "src/y2019/d5.txt" [5]))
        all-passed (every? zero? (rest (:output state)))
        output0 (first (:output state))]
    (if (println (str "Passed all diagnostic tests. Part 2 is " output0))
      all-passed
      (println "Did not pass diagnostic tests."))
    output0))