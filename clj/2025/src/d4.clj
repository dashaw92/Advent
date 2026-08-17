;; Read file `f` into a grid (2D array) of chars.
(defn read-grid [f]
  (as-> (slurp f) $
    ;; Split on newlines to create the y-axis of the grid.
    (clojure.string/split $ #"\n")
    ;; Split each line individually as chars (x-axis of the grid).
    (map #(clojure.string/split % #"") $)
    ;; Convert the outer and inner elements to persistent vectors so the entire grid acts like a 2D array via indexing [y][x].
    (mapv vec $)))

;; All possible neighbors at a given position [x y].
;; @ @ @
;; @ X @
;; @ @ @
(defn neighbors [[x y]]
  [[(dec x), y]
   [(inc x), y]
   [(dec x), (dec y)]
   [(dec x), (inc y)]
   [(inc x), (dec y)]
   [(inc x), (inc y)]
   [x, (dec y)]
   [x, (inc y)]])

(defn bounds [grid]
  (let [width (count (first grid))
        height (count grid)]
    [width height]))

;; All possible positions within the grid as coordinate-pairs.
(defn all-pos [grid]
  (let [[w h] (bounds grid)
        all (for [y (range 0 h)
                  x (range 0 w)] [x y])]
    all))

;; Get the cell at [x y] in the grid
(defn at-grid [grid [x y]]
  (-> grid
       (nth y)
       (nth x)))

;; If the position is in bounds and the cell at that position is paper ("@").
(defn is-paper [grid [x y]]
  (let [[w h] (bounds grid)]
    (and (>= x 0) (< x w) (>= y 0) (< y h) (= "@" (at-grid grid [x y])))))

;; If the position is paper and has less than 4 neighboring paper cells.
(defn is-accessible [grid [x y]]
  (let [my-neighbors (neighbors [x y])
        paper (count (filter (partial is-paper grid) my-neighbors))]
    (and (is-paper grid [x y]) (<= paper 3))))

;; Count how many paper cells are accessible
(defn count-accessible-paper [grid]
  (let [all (all-pos grid)]
    (count (filter (partial is-accessible grid) all))))

;; Create a new grid where the position [x y] is set to non-paper.
(defn remove-paper-at [grid [x y]]
  (update-in grid [y x] (constantly ".")))

;; Create a new grid with all previously accessible paper is removed.
(defn remove-accessible [grid]
  (let [all (all-pos grid)
        all-paper (filter (partial is-accessible grid) all)]
    (reduce remove-paper-at grid all-paper)))

;; Just count accessible paper in the original grid.
(defn solve-p1 [f]
  (let [grid (read-grid f)
        accessible (count-accessible-paper grid)]
    accessible))

;; While there is accessible paper, keep generating new grids with all
;; accessible paper removed (and track how much was removed each round).
;; Yield the total number of paper removed across all grids when
;; a grid no longer has any accessible paper.
(defn solve-p2 [f]
  (loop [grid (read-grid f)
         removed 0]
    (let [accessible (count-accessible-paper grid)]
      (if (= accessible 0)
        removed
        (recur (remove-accessible grid) (+ removed accessible))))))

(solve-p1 "d4.txt")
(solve-p2 "d4.txt")
