(ns y2019.day5
  (:require [clojure.string :as str]))

(defn bind-input [state input]
  (assoc state :input input))

(defn lit->state
  ([lit] (as-> lit $
               (str/split $ #",")
               (map ^[String] Integer/parseInt $)
               (assoc {} :tape (vec $))))
  ([lit input] (bind-input (lit->state lit) input)))

(defn read-state
  "Creates an Intcode state from a text file containing integers separated by commas."
  ([f] (-> f
           slurp
           lit->state))
  ([f input] (bind-input (read-state f) input)))


;Functions for Intcode opcodes take 2 arguments:
;[[args...] state]
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

(defn read-in
  "'Reads' a single integer and stores it at [`addr`]. Input is popped from
  the front of the :input vec in state."
  [[addr] state]
  (let [input (first (:input state))
        rest (rest (:input state))
        replaced-state (assoc-in state [:tape addr] input)]
    (when (nil? input) (throw (IllegalStateException. "Tried to read input but :input was empty.")))
    (assoc replaced-state :input rest)))

(defn write-out
  "Outputs the value at the address (pushes it onto the :output stack)"
  [[addr] state]
  (let [val (nth (:tape state) addr)]
    (assoc state :output (cons val (:output state)))))

(def opcodes
  "Map of opcode literals to parameters: :len and :func.
  :len is how many arguments the opcode requires.
  :func is the opcode function to call when encountering this opcode (signature: [opcode [args] state])"
  {1  {:len 3 :func (partial binop +)}
   2  {:len 3 :func (partial binop *)}
   3  {:len 1 :func read-in}
   4  {:len 1 :func write-out}
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
      (if (empty? rest)
        next-state
        (recur next-state rest)))))

;(defn solve
;  "This puzzle requires you to overwrite the values at cells 1 and 2 in the input
;  tape to specific values. Pass the map of overwrites as a vec of maps with
;  keys :cell and :val corresponding to the index and value to set. Afterwards,
;  return the value of the cell at position 0 in the tape."
;  [f overwrites]
;  (let [init-state (read-state f)
;        overwritten-state (overwrite-cells init-state overwrites)
;        output-state (run overwritten-state)
;        pos0 (first (:tape output-state))]
;    pos0))

(run (lit->state "3,0,4,0,99" [0]))