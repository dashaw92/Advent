(ns y2019.day7
  (:require [y2019.intcode :as intcode]
            [clojure.math.combinatorics :as combo]))

(intcode/read-state "src/y2019/d7.txt")
(defn run-amps [init-state phases]
  (loop [output 0
         phases phases]
    (let [phase (first phases)
          phases (rest phases)
          state (intcode/bind-input init-state [phase output])
          exec (intcode/run state)
          result (first (:output exec))]
      (if (empty? phases)
        result
        (recur result phases)))))

;(run-amps (intcode/lit->state "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0") [1 0 4 3 2])
(defn solve [f]
  (let [state (intcode/read-state f)
        phases (combo/permutations [0 1 2 3 4])
        outputs (for [phase phases]
                  (run-amps state phase))
        max-output (apply max outputs)]
    max-output))

(solve "src/y2019/d7.txt")