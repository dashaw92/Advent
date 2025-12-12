(defn split-mid [s]
  (let [mid (/ (count s) 2)
        a (subs s 0 mid)
        b (subs s mid)]
    [a b]))

(defn is-invalid [iid]
  (let [[a b] (split-mid iid)]
    (= a b)))

; "93-97" -> [93, 94, 95, .., 97]
(defn all-ids [form]
  (let [[_          start end]
        (re-find #"(\d+)-(\d+)" form)
        start (Long/parseLong start)
        end (Long/parseLong end)
        all (for [id (range start (inc end))] (str id))]
    all))

(defn read-input [file]
  (as-> file $
    (slurp $)
    (clojure.string/split $ #",")
    (mapcat all-ids $)
    (filter is-invalid $)
    (map #(Long/parseLong %) $)
    (reduce + 0 $)))

(println (read-input "d2.txt"))
