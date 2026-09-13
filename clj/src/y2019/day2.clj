(ns y2019.day2
  (:require [clojure.string :as str]))

(defn read-state
  "Creates an Intcode state from a text file containing integers separated by commas."
  [f]
  (as-> f $
        (slurp $)
        (str/split $ #",")
        (map Integer/parseInt $)
        (assoc {} :tape (vec $))))

;Functions for Intcode opcodes take 3 arguments:
;[opcode [args...] state]
;args is a variable length vec of arguments required by the opcode
;state is the entire state map with the program tape in :tape
;The function should yield the next state as a map with the new tape at :tape (assoc state :tape (modify (:tape state)))

(defn binop
  "Binary operations where the only code that changes between opcodes is the function applied to
  the operands. The state map will be updated with the result of the operation provided."
  [op [in1 in2 out] state]
  (let [tape (:tape state)
        in1 (nth tape in1)
        in2 (nth tape in2)
        result (op in1 in2)
        tape' (assoc tape out result)]
    (assoc state :tape tape')))

(defn halt
  "Ignores the first argument to match the shape of opcode functions.
  Adds the :halted flag to the state to indicate the program has finished."
  [_ state] (assoc state :halted true))

(def opcodes
  "Map of opcode literals to parameters: :len and :func.
  :len is how many arguments the opcode requires.
  :func is the opcode function to call when encountering this opcode (signature: [opcode [args] state])"
  {1  {:len 3 :func (partial binop +)}
   2  {:len 3 :func (partial binop *)}
   99 {:len 0 :func halt}})

(defn op+args
  "Use the opcodes map to extract the next operation and its arguments from the tape.
  Throws on invalid (unknown) opcodes. Returns a vec of [opcode (args...)]"
  [state pc]
  (let [tape (:tape state)
        current (nth tape pc nil)
        rest (drop (inc pc) tape)
        args (take (:len (opcodes current)) rest)]
    (if (not (contains? opcodes current))
      (throw (IllegalArgumentException. (str "Invalid opcode " current)))
      (cons current args))))

(defn run-step
  "Execute a single step of the Intcode program: state -> state'"
  [[opcode & args] state]
  (let [args (vec args)
        func (:func (opcodes opcode))]
    (func args state)))

(defn run
  "Execute the Intcode program until it either halts (via :halted) or runs out of instructions (pc > count).
  Pass a marker argument (idiomatically :debug) to this function to print debug information."
  [state & debug?]
  (loop [pc 0
         state state]
    (when debug? (println pc state))
    (if (contains? state :halted)
      state
      (let [instrs (op+args state pc)]
        (when debug? (println instrs))
        (if (nil? (first instrs))
          state
          (recur (+ pc (count instrs)) (run-step instrs state)))))))

(defn overwrite-cells
  "Overwrite cells at :cell x with :val y (via vec of maps)."
  [state cell->values]
  (loop [state state
         cell->values cell->values]
    (let [next (first cell->values)
          rest (rest cell->values)
          next-state (assoc-in state [:tape (:cell next)] (:val next))]
      (if (empty? cell->values)                             ;note: checking `rest` here would cause the final map to be ignored
        state
        (recur next-state rest)))))

(defn solve-p1
  "Part 1 requires you to overwrite the values at cells 1 and 2 in the input
  tape to specific values. Pass the map of overwrites as a vec of maps with
  keys :cell and :val corresponding to the index and value to set. Afterward,
  return the value of the cell at position 0 in the tape."
  [f overwrites]
  (let [init-state (read-state f)
        overwritten-state (overwrite-cells init-state overwrites)
        output-state (run overwritten-state)
        pos0 (first (:tape output-state))]
    pos0))

(solve-p1 "src/y2019/d2.txt" [{:cell 1 :val 12} {:cell 2 :val 2}])