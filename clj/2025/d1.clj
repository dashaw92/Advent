(defn to-move [raw]
  (let [[_ dir step]
        (re-find #"(R|L)(\d+)" raw)
        step (Integer/parseInt step)]
    (if (= "L" dir) (- step) step)))

(defn read-input [file]
  (as-> file $
    (slurp $)
    (clojure.string/split $ #"\n")
    (mapv to-move $)))

;[current occur] = state
;current = int, current location on the dial
;occur = count of dial states seen
(defn step [[current occur] step]
  (let [next (mod (+ current step) 100)
        nextOccur (update occur next (fnil inc 0))]
    [next nextOccur]))

(def solved (reduce step [50 {50 1}] (read-input "d1e.txt")))
(def part1 ((nth solved 1) 0))

part1
