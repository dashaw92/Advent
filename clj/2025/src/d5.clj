(def split clojure.string/split)

(defn read [file]
  (slurp file))

;; "3-5" -> {:start 3 :end 5}
(defn parse-range [range]
  (let [[_ start end] (re-find #"(\d+)-(\d+)" range)
        start (Long/parseLong start)
        end (Long/parseLong end)]
    {:start start :end end}))

;; Split the input into two parts (partitioned by "\n\n"): a seq of ranges (see parse-range) and a seq of ints
;; output is a map {:ranges (seq of ranges) :ings (seq of ints)}
(defn read-input [file]
  (let [[ranges ings] (split (read file) #"\n\n")
        ranges (split ranges #"\n")
        ings (split ings #"\n")
        ranges (map parse-range ranges)
        ings (map #(Long/parseLong %) ings)]
    {:ranges ranges :ings ings}))

;; If a number is within a range. Inclusive on both bounds.
(defn in-range [range ing]
  (and (>= ing (:start range)) (<= ing (:end range))))

;; Check all given ranges to see if the number is within any.
(defn is-in-any-range [ranges ing]
  (some #(in-range % ing) ranges))

;; Count the number of ingredients within any of the ranges.
(defn solve-p1 [file]
  (let [input (read-input file)
        check-ranges (partial is-in-any-range (:ranges input))
        fresh (filter check-ranges (:ings input))]
    (count fresh)))

;; Sort all ranges based off the end bound. Required to properly merge ranges.
(defn sort-ranges [ranges]
  (sort-by :end ranges))

;; The naive approach to part 2 is to check each range for inclusivity incrementally.
;; This works fine for the example input, but is beyond impractical for the real input (many ranges that are extremely large in number space.).
;; The solution is to merge the ranges so that any overlaps between ranges are converted into a single new range with the updated bounds based off the previous and current.
(defn merge-range [[old r] next]
  (cond
    ;; If the next range is entirely contained within the current range (r), skip next entirely
    (and (>= (:start next) (:start r)) (<= (:end next) (:end r))) [old r]
    ;; If the old range is entirely contained within the next range, replace it with next
    (and (>= (:start r) (:start next)) (<= (:end r) (:end next))) [old next]
    ;; If the next range starts at the end of the old one (or is + 1), update the old range with the new bounds.
    (<= (dec (:start next)) (:end r)) [old {:start (:start r) :end (:end next)}]
    ;; Cannot merge these ranges, update the list of finished ranges (old) and return next.
    :else [(conj old r) next]))

;; Count how many elements a range represents (inclusive of both bounds).
(defn count-range [r]
  (inc (- (:end r) (:start r))))

;; Repeatedly sort and merge ranges until nothing happens (all possible merges are completed).
;; Total up the amount the remaining ranges represent.
(defn solve-p2 [file]
  (let [input (read-input file)]
    (loop [sorted (sort-ranges (:ranges input))]
      (let [[old r] (reduce merge-range [[] (first sorted)] (rest sorted))
            ranges (conj old r)
            next (sort-ranges ranges)]
        (if (= (count sorted) (count next))
              (reduce + 0 (map count-range ranges))
              (recur next))))))

(solve-p1 "d5.txt")
(solve-p2 "d5.txt")
