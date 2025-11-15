(ns y2015.d1)

;; filename: String -> char[]
(defn get-input 
  "Read the file into a vector of characters"
  [file] 
  (->> file
       slurp
       vec))


(def input (get-input "resources/day1.txt"))

;; Translates the input from the ( and ) "stairs" into absolute
;; "up" (1) or "down" (-1) instructions which can be summed via reduce (or reductions)
(defn remap-floors [input]
  (map {\( 1 \) -1} input))

;; Simple sum- final output is the final stair level Santa ends his journey on
(def part1 (reduce + (remap-floors input)))
;; reductions yields a new sequence where each index is replaced by the function's application
;; as it traverses through the original sequence. With the `remap-floors` sequence,
;; each step is a running accumulation- where Santa is at that index. From there, just
;; find the first index of -1, or where Santa entered the basement levels for the first
;; time.
;; #(not= -1 %1) is a short-form anonymous function, and could be rewritten as:
;; (fn [arg] (not= -1 arg)), but obviously #(...) is easier to read. %1 is the argument.
;; The index found here from count'ing the result of take-while is further inc'ed because
;; the problem wants the index of the first -1 and take-while is end-exclusive.
(def part2 (inc (count (take-while #(not= -1 %1) (reductions + (remap-floors input))))))
(println "Part 1: " part1)
(println "Part 2: " part1)
