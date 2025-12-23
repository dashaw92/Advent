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

(defn convert-p2 [record]
  (let [nums (butlast record)
        op (last record)
        longest (map count nums)
        how-many (apply max longest)
        new-nums (for [i (range how-many)]
                   (map #(nth % i nil) nums))
        new-nums (map (partial apply str) new-nums)
        output (reverse (conj new-nums op))]
    (println output)
    output))

(defn solve [records]
  (->> records
   (map #(apply (:op %) (:nums %)))
   (reduce + 0)))

;; reduce each equation in the input using the op from each
;; sum the results for part 1
(defn solve-p1 [file]
  (->> file
   (lines)
   (map to-p1)
   (solve)))

(defn solve-p2 [file]
  (->> file
   (lines)
   (map convert-p2)
   (map to-p1)
   (solve)))

(solve-p1 "d6e.txt")
;; this almost works except the alignment matters. this solution always assumes left alignment which throws off the answer by a ton.
(solve-p2 "d6e.txt")
