(ns y2015.core)

(defn get-input 
  "Read the file into a vector of characters"
  [file] 
  (->> file
       slurp
       vec))

(defn final-floor 
  [input]
  (letfn [(step [input current] ;letfn gets rid of a warning "inline def" (defn)... 
            (case (first input) 
              \( (recur (rest input) (inc current))
              \) (recur (rest input) (dec current))
              nil current))]
    (step input 0)))

(defn find-first-basement
  [input]
  (letfn [(step [input current index]
                (if (= current -1)
                  index
                  (case (first input)
                    \( (recur (rest input) (inc current) (inc index))
                    \) (recur (rest input) (dec current) (inc index)))))]
    (step input 0 0)))

(def input (get-input "resources/day1.txt"))
(println (str "Part 1: " (final-floor input)))
(println (str "Part 2: " (find-first-basement input)))