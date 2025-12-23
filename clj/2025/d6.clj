;; find all instances of char `a` in string `s` as a sequence
(defn find-all [s a]
         ;; index to start searching from
  (loop [start 0
         ;; previous indices found
         acc []]
    (let [next (.indexOf s a start)]
      ;; -1 means no more found from the starting position
      (if (= -1 next)
        acc
        ;; inc because the start index is inclusive so indexOf will keep finding the current index.
        (recur (inc next) (conj acc next))))))

;; `edges` is a pair of boundaries (`start` and `end`) representing the edges of a column from the input.
;; converts all edges into substring calls on the row to extract fields with padding intact.
(defn apply-substring [edges row]
  (for [i (range (count edges))]
      (let [r (nth edges i)
            start (nth r 0)
            end (nth r 1)]
        (.substring row start end))))

;; The numbers are lined up arbitrarily, but the operators are always aligned to the left.
;; From this, it's possible to find the width of every column in the input and extract each field
;; while retaining padding.
(defn extract-fields [all-rows]
  (let [ops (last all-rows)
        ;; this will be paired up to the last edge so the substring step doesn't have any edge cases.
        total-len (count ops)
        ;; create a sequence of indexes where operators are located in the ops line.
        plus (find-all ops "+")
        times (find-all ops "*")
        ops (apply conj plus times)
        ;; sliding window of indexes where each pair is a boundary for a substring call to extract fields exactly
        ;; from the rows while retaining padding.
        edges (partition 2 1 (sort (conj ops total-len)))]
    ;; use the found edges to extract every field from all rows, including the operators row.
    (map (partial apply-substring edges) all-rows)))

;; reorients the input to match the vertical alignment of numbers / ops.
(defn transpose [all]
  (apply map list all))

;; reads all lines, extracts all fields, and transposes them so each row is a column from the input
(defn lines [file]
  (as-> file $
    (slurp $)
    (clojure.string/split $ #"\n")
    (extract-fields $)
    (transpose $)))

;; the input is a list of numbers concluded by a single op (*, -, +, /)
;; convert the numbers to ints and the op to a function that accepts those ints
(defn to-p1 [record]
  (let [nums (butlast record)
        nums (map (comp #(Integer/parseInt %) #(.trim %)) nums)
        op (.trim (last record))
        ;; ops in this puzzle are only '*' or '+'
        op ({"+" + "*" *} op)]
    {:nums nums :op op}))

;; read the numbers right to left by columns
;; 64
;; 23
;; 314 
;; -> [4 431 623]
(defn convert-p2 [record]
  (let [nums (butlast record)
        op (last record)
        ;; all fields are padded to the same width in any column,
        ;; so use the convenient op field as the width of this column.
        width (count op)
        ;; extract the ith char from each element of nums based off width
        new-nums (for [i (range width)]
                   (map #(nth % i) nums))
        ;; join each sequence of newly-extracted nums back to a string
        new-nums (map (partial apply str) new-nums)
        ;; some results might be empty. remove them
        ;; also convert to vec so conj will add to the end (conj adds to the front of lists)
        new-nums (filterv (fn [s] (not (.isBlank s))) new-nums)
        ;; add op back to the record at the end
        output (conj new-nums op)]
    output))

;; {:op `op` :nums `nums` } -> nums[0] `op` nums[1] `op` ... 
(defn solve [records]
  (->> records
   (map #(apply (:op %) (:nums %)))
   (reduce + 0)))

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
(solve-p2 "d6e.txt")
