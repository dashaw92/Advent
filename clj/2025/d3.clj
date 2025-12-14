;; convert ASCII numbers to decimal
;; \0 -> 0
;; don't pass in non-numeric characters and expect anything useful
(defn char-to-num [x]
  (- (int x) (int \0)))

;; Read f and split on newlines
(defn lines [f]
  (-> f
      (slurp)
      (clojure.string/split #"\n")))

;; Create all possible slices of the sequence that have a tail
;; Sort by the first element, find the slice which has the largest
;; value first
(defn largest-with-tail [bats]
  (let [slices (for [x (range 2 (inc (count bats)))] (take-last x bats))
        sorted (reverse (sort-by first slices))
        largest (first sorted)]
    largest))

;; Find the next max value in a slice ignoring the first element
(defn next-largest [slice]
  (let [next-max (apply max (rest slice))]
    next-max))

;; [1 3] -> 13
(defn bats-to-num [[a b]]
  (+ (* a 10) b))

;; Given a battery bank, find the largest two digit number that can be
;; derived from the individual batteries in the bank
;; 134982 -> 98
(defn largest-bats [bats]
  (let [first-max (largest-with-tail bats)
        a (first first-max)
        b (next-largest first-max)
        out (bats-to-num [a b])]
    out))

(defn solve-p1 []
  (->> (lines "d3.txt")
     (map #(map char-to-num %))
     (map largest-bats)
     (reduce + 0)))

(solve-p1)
