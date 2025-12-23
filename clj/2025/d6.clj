(defn lines [file]
  (as-> file $
    (slurp $)
    (clojure.string/split $ #"\n")
    (map extract-fields $)
    (transpose $)))

;; reorients the input to match the vertical alignment of numbers / ops.
(defn transpose [all]
  (apply map list all))

;; ignore all spaces and extract operators and operands from input records
(defn extract-fields [record]
  (let [fields (re-seq #"([\*\-\+\/]|\d+)[ ]*" record)]
    (map last fields)))

;; the input is a list of numbers concluded by a single op (*, -, +, /)
;; convert the numbers to ints and the op to a function that accepts those ints
(defn to-p1 [record]
  (let [nums (butlast record)
        nums (map #(Integer/parseInt %) nums)
        op (last record)
        op ({"+" + "*" * "/" / "-" -} op)]
    {:nums nums :op op}))

;; reduce each equation in the input using the op from each
;; sum the results for part 1
(defn solve-p1 [file]
  (->>
   (map to-p1 (lines file))
   (map #(apply (:op %) (:nums %)))
   (reduce + 0)))

(solve-p1 "d6.txt")
