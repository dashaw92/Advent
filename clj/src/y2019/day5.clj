(ns y2019.day5
  (:require [clojure.string :as str]))

(defn bind-input [state input]
  (assoc state :input input))

(defn lit->state
  ([lit] (as-> lit $
               (str/split $ #",")
               (map ^[String] Integer/parseInt $)
               (assoc {} :tape (vec $) :ip 0)))
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

;Functions for Intcode opcodes take 2 arguments:
;args is a variable length vec of arguments required by the opcode
;state is the entire state map with the program tape in :tape
;The function should yield the next state as a map with the new tape at :tape (assoc state :tape (modify (:tape state)))

(defn binop
  "Binary operations where the only code that changes between opcodes is the function applied to
  the operands. The state map will be updated with the result of the operation provided."
  [op [in1 in2 out] state]
  (let [tape (:tape state)
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
  (assoc state :output (cons addr (:output state))))

(defn jump
  "Set state IP to in2 if (pred in1). Do nothing otherwise"
  [pred [in1 in2] state]
  (if (pred in1)
    (assoc state :ip in2 :jumped true)
    state))

(defn cmp
  "If (cmp-fn in1 in2), store 1 at out. Otherwise stores 0."
  [cmp-fn [in1 in2 out] state]
  (if (cmp-fn in1 in2)
    (assoc-in state [:tape out] 1)
    (assoc-in state [:tape out] 0)))

(def opcodes
  "Map of opcode literals to parameters: :len and :func.
  :params explains how the opcode uses its arguments (and how many arguments it has).
    a :r argument is an argument the opcode reads from the tape and can be subject to parameter modes (:position or :immediate)
    a :w argument is an output for the opcode and is *always* in :position mode
  :func is the opcode function to call when encountering this opcode (signature: [opcode [args] state])"
  {1  {:params [:r :r :w] :func (partial binop +)}
   2  {:params [:r :r :w] :func (partial binop *)}
   3  {:params [:w] :func read-in}
   4  {:params [:r] :func write-out}
   5  {:params [:r :r] :func (partial jump (partial not= 0))}
   6  {:params [:r :r] :func (partial jump (partial zero?))}
   7  {:params [:r :r :w] :func (partial cmp <)}
   8  {:params [:r :r :w] :func (partial cmp =)}
   99 {:params [] :func halt}})

(defn op+args
  "Use the opcodes map to extract the next operation and its arguments from the tape.
  Throws on invalid (unknown) opcodes. Returns a vec of [opcode (args...)]"
  [state pc]
  (let [tape (:tape state)
        current (nth tape pc nil)
        {:keys [opcode params]} (extract-arg-modes current)
        rest (drop (inc pc) tape)
        arg-modes (:params (opcodes opcode))
        args (take (count arg-modes) rest)
        params (for [i (range (count arg-modes))
                     :let [func-mode (nth arg-modes i)
                           opcode-mode (nth params i)
                           arg (nth args i)]]
                 (if (= :w func-mode)
                   arg
                   (read-addr tape opcode-mode arg)))]
    (if (not (contains? opcodes opcode))
      (throw (IllegalArgumentException. (str "Invalid opcode " opcode)))
      (cons opcode params))))

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
  (loop [state state]
    (when debug? (println (:ip state) state))
    (if (contains? state :halted)
      state
      (let [ip (:ip state)
            instrs (op+args state ip)]
        (when debug? (println instrs))
        (if (nil? (first instrs))
          state
          (let [next-state (run-step instrs state)
                ip (:ip next-state)
                next-ip (if (contains? next-state :jumped) ip (+ ip (count instrs)))
                reset-jmp-state (dissoc next-state :jumped)
                state (assoc reset-jmp-state :ip next-ip)]
            (recur state)))))))

(run (lit->state "1002,4,3,4,33" [0]))                      ;correctly halts {:tape [1002 4 3 4 99] :halted true ..}
(def p1
  (let [state (run (read-state "src/y2019/d5.txt" [1]))
        all-passed (every? zero? (rest (:output state)))
        output0 (first (:output state))]
    (if all-passed
      (println (str "Passed all diagnostic tests. Part 1 is " output0))
      (println "Did not pass diagnostic tests."))
    output0))
(def p2
  (let [state (run (read-state "src/y2019/d5.txt" [5]))
        all-passed (every? zero? (rest (:output state)))
        output0 (first (:output state))]
    (if all-passed
      (println (str "Passed all diagnostic tests. Part 2 is " output0))
      (println "Did not pass diagnostic tests."))
    output0))