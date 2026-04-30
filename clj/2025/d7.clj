(defn lines [f]
  (-> f
      (slurp)
      (clojure.string/split #"\n")))

(defn splitters [line]
  (loop [i 0
         acc []]
    (let [next (.indexOf line "^" i)]
      (if (= -1 next)
        acc
        (recur (inc next) (conj acc next))))))

(defn start-pos [line]
  (.indexOf line "S"))

(def input (lines "d7e.txt"))

(defn next-beam-state [ss beam]
  (println ss beam)
  (if (some (partial = beam) ss)
    [(dec beam) (inc beam)]
    [beam]))

(defn run [input]
  (loop [i 1
         line (nth input i nil)
         beams [(start-pos (nth input 0))]]
    (if (nil? line)
      beams
      (let [s (splitters line)
          next (map (partial next-beam-state s) beams)]
        (recur (inc i) (nth input (inc i) nil) (mapcat conj next))))))

(count (run input))
(mapcat conj (map (partial next-beam-state [7 1]) [7 1 3]))
