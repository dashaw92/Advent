;; This buffer is for text that is not saved, and for Lisp evaluation.
;; To create a file, visit it with ‘C-x C-f’ and enter text in its buffer.

(defn read-input [file]
  (as-> file $
    (slurp $)
    (clojure.string/split $ #"\n\n")
    (mapv #(clojure.string/split %1 #"\n") $)))

(defn parse-line [line]
  (as-> line $pipe
    (clojure.string/split $pipe #",")
    (mapv #(Integer/parseInt %) $pipe)))

(defn extract-rule [input]
  (let [[_ fst snd]
        (re-matches #"(\d+)\|(\d+)" input)
        fst (Integer/parseInt fst)
        snd (Integer/parseInt snd)
        ]
    [fst snd]))

(defn has-pages [line [fst snd]]
  (and (>= (.indexOf line fst) 0) (>= (.indexOf line snd) 0)))

(defn in-order [line [fst snd]]
  (or (not (has-pages line [fst snd])) (< (.indexOf line fst) (.indexOf line snd))))

(defn all-in-order [line rules]
  (every? #(in-order line %) rules))

(defn middle-page [line]
  (let [l (count line)
        m (/ l 2)
        middle (nth line m)]
    middle))

(defn solve [input]
  (let [[rules pages] input
        rules (map extract-rule rules)
        pages (map parse-line pages)
        validPages (filter #(all-in-order % rules) pages)
        middlePages (map middle-page validPages)
        solution (reduce + 0 middlePages)
        ]
    solution))  

(def input (read-input "input2.txt"))
(solve input)
