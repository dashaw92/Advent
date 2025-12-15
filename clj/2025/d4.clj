(defn read-grid [f]
  (as-> (slurp f) $
    (clojure.string/split $ #"\n")
    (map #(clojure.string/split % #"") $)))

(defn neighbors [x y]
  )
