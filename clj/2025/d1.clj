(defn to-move [raw]
  ""
  (let [[_ dir step]
        (re-find #"(R|L)(\d+)" raw)
        step (Integer/parseInt step)]
    {:dir (keyword dir) :step step}))

(defn read-input [file]
  (as-> file $
    (slurp $)
    (clojure.string/split $ #"\n")
    (map to-move $)))

(defn step [[current p1 p2] step]
  "delta: convert :L to -1 and :R to 1
  n: stopping point based off delta and the step (end-inclusive via inc), i.e. {:dir :L :step 30} -> n = -30, {:dir :R :step 30} -> n = 30
  steps: every step of the rotation applied to the current value modulo 100
  outP1: If the last step a rotation ends on a 0, increment part 1
  outP2: Count the amount of times 0 was seen as part of a rotation, excluding the starting position."
  (let [delta (if (= (:dir step) :L) (- 1) 1)
        n (* (inc (:step step)) delta)
        steps (for [i (range 0 n delta)] (mod (+ current i) 100))
        outP1 (if (= (last steps) 0) 1 0)
        outP2 (count (filter #(= 0 %) (rest steps)))]
    ; the last element in steps is the starting position for the next move
    [(last steps) (+ p1 outP1) (+ p2 outP2)]))

(def solved (reduce step [50 0 0] (read-input "d1.txt")))
; [_ part1 part2]
solved
