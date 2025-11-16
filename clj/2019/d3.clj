(def split clojure.string/split)
(def intersect clojure.set/intersection)

(defn parse-instr [s]
  (let [[_           dir      amt]
        (re-find #"(U|R|D|L)(\d+)" s)
        dir (keyword dir)
        amt (Integer/parseInt amt)]
    {:dir dir :amt amt}))

(defn to-instrs [line]
  (let [instrs (split line #",")
        instrs (map parse-instr instrs)]
    instrs))

(defn get-input [file]
  (as-> file $
      (slurp $)
      (split $ #"\n")
      (map to-instrs $)))

(get-input "ex1.txt")

(defn delta [{:keys [dir amt]} [x y]]
  "Generate the end position, axis, and step delta for any given instruction"
  (case dir
        :U [x (- y amt) :y -1]
        :D [x (+ y amt) :y 1]
        :L [(- x amt) y :x -1]
        :R [(+ x amt) y :x 1]))

(defn run-instr [state instr [x y]]
  "Generate all steps the current instr will take given the current (x, y)"
  (let [[dx dy dir step] (delta instr [x y])
        cells (case dir
                :y (for [iy (range y dy step)] [x iy])
                :x (for [ix (range x dx step)] [ix y]))]
    [(into state cells) (last cells)]))

(defn reduce-wire [[state lastPos] instr]
  "Wire run-instr into a reduce call"
  (run-instr state instr lastPos))

(defn run-wire [wire]
  "Reduce the wire's instructions into [visited cells, last position]"
  (reduce reduce-wire [#{} [0 0]] wire))

(defn dist [[x y]]
  "Manhattan distance between (0, 0) and (x, y)"
  (+ (abs (- 0 x)) (abs (- 0 y))))

(->> (get-input "input.txt")
     (map run-wire)
     (map first)
     (reduce intersect)
     (filter #(not= % [0 0])) ;;origin doesn't count
     (map dist)
     (apply min))
