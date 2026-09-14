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

(defn read-addr
  "Handles reading from the tape with support for all argument modes."
  [tape mode addr]
  (case mode
    :position (nth tape addr)
    :immediate addr))

(defn extract-arg-modes [opcode]
  (let [base-opcode (mod opcode 100)
        params (take-last 3 (concat (repeat 3 \0) (drop-last 2 (str opcode))))
        to-mode #(if (= \0 %) :position :immediate)
        params (map to-mode params)]
    {:opcode base-opcode :params (reverse params)}))

;Functions for Intcode opcodes take 3 arguments:
;[[param0 param1 param2] [args...] state]
;paramN = mode of parameter at index N in args (:position or :immediate)
;args is a variable length vec of arguments required by the opcode
;state is the entire state map with the program tape in :tape
;The function should yield the next state as a map with the new tape at :tape (assoc state :tape (modify (:tape state)))

(defn binop
  "Binary operations where the only code that changes between opcodes is the function applied to
  the operands. The state map will be updated with the result of the operation provided."
  [op [param1 param2 _] [in1 in2 out] state]
  (let [tape (:tape state)
        in1 (read-addr tape param1 in1)
        in2 (read-addr tape param2 in2)
        result (op in1 in2)
        tape' (assoc tape out result)]
    (assoc state :tape tape')))

(defn halt
  "Ignores the first two arguments to match the shape of opcode functions.
  Adds the :halted flag to the state to indicate the program has finished."
  [_ _ state] (assoc state :halted true))

(defn read-in
  "'Reads' a single integer and stores it at [`addr`]. Input is popped from
  the front of the :input vec in state."
  [_ [addr] state]
  (let [input (first (:input state))
        rest (rest (:input state))
        replaced-state (assoc-in state [:tape addr] input)]
    (when (nil? input) (throw (IllegalStateException. "Tried to read input but :input was empty.")))
    (assoc replaced-state :input rest)))

(defn write-out
  "Outputs the value at the address (pushes it onto the :output stack)"
  [[param0 _ _] [addr] state]
  (let [val (read-addr (:tape state) param0 addr)]
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
        {:keys [opcode params]} (extract-arg-modes current)
        rest (drop (inc pc) tape)
        args (take (:len (opcodes opcode)) rest)]
    (if (not (contains? opcodes opcode))
      (throw (IllegalArgumentException. (str "Invalid opcode " opcode)))
      (cons [opcode params] args))))

(defn run-step
  "Execute a single step of the Intcode program: state -> state'"
  [[[opcode params] & args] state]
  (let [args (vec args)
        func (:func (opcodes opcode))]
    (func params args state)))

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

(run (lit->state "1002,4,3,4,33" [0])) ;correctly halts {:tape [1002 4 3 4 99] :halted true ..}
(def p1
  (let [state (run (read-state "src/y2019/d5.txt" [1]))
        all-passed (every? zero? (rest (:output state)))
        output0 (first (:output state))]
    (if all-passed
      (println (str "Passed all diagnostic tests. Part 1 is " output0))
      (println "Did not pass diagnostic tests."))
    output0))