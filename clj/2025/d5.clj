(defn read [file]
  (slurp file))

(defn parse-range [range]
  (let [[_ start end] (re-find #"(\d+)-(\d+)" range)
        start (Long/parseLong start)
        end (Long/parseLong end)]
    {:start start :end end}))

(defn read-input [file]
  (let [[ranges ings] (clojure.string/split (read file) #"\n\n")
        ranges (clojure.string/split ranges #"\n")
        ings (clojure.string/split ings #"\n")
        ranges (map parse-range ranges)
        ings (map #(Long/parseLong %) ings)]
    {:ranges ranges :ings ings}))

(defn in-range [range ing]
  (and (>= ing (:start range)) (<= ing (:end range))))

(defn is-in-any-range [ranges ing]
  (some #(in-range % ing) ranges))

(defn solve [file]
  (let [input (read-input file)
        check-ranges (partial is-in-any-range (:ranges input))
        fresh (filter check-ranges (:ings input))]
    (count fresh)))

(solve "d5.txt")
