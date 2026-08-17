;; convert ASCII numbers to decimal
;; \0 -> 0
;; don't pass in non-numeric characters and expect anything useful
(defn char-to-num [x]
  (- (long x) (long \0)))

;; Read f and split on newlines
(defn lines [f]
  (-> f
      (slurp)
      (clojure.string/split #"\n")))

;; Create all possible slices of the sequence that have a tail
;; Sort by the first element, find the slice which has the largest
;; value first. Split this slice into [first rest...]
(defn largest-with-tail [min-size bats]
  (let [slices (for [x (range min-size (inc (count bats)))] (take-last x bats))
        sorted (reverse (sort-by first slices))
        largest (first sorted)]
    [(first largest) (rest largest)]))

;; Find the next max value in a slice. Terminal operation for this puzzle
;; This edge case is required because largest-with-tail only yields slices
;; of length > 1.
(defn next-largest [slice]
  (let [next-max (apply max slice)]
    next-max))

;; [1 3] -> 13
(defn bats-to-num [[a b]]
  (+ (* a 10) b))

;; Given a battery bank, find the largest `iters`-digit number that can be
;; derived from the individual batteries in the bank
;; 134982:
;;   iters = 2 -> 98
;;   iters = 3 -> 982
;;   iters = 4 -> 4982
(defn largest-bats [iters bats]
  (let [
        [a remaining]
        (loop
            [acc 0
             i (dec iters)
             [a next-slice] (largest-with-tail iters bats)]
          (if (= 1 i)
            [(bats-to-num [acc a]) next-slice]
            (recur (bats-to-num [acc a]) (dec i) (largest-with-tail i next-slice))))
        b (next-largest remaining)
        out (bats-to-num [a b])]
    out))

(defn solve [f iters]
  (->> (lines f)
     (map #(map char-to-num %))
     (map (partial largest-bats iters))
     (reduce + 0)))

(solve "d3.txt" 2)
(solve "d3.txt" 12)
