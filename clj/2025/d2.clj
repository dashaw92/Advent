; "asdf" -> ["as" "df"]
(defn split-mid [s]
  (let [mid (/ (count s) 2)
        a (subs s 0 mid)
        b (subs s mid)]
    [a b]))

; "1212" -> "12" == "12"
; "1213" -> "12" != "13"
(defn is-invalid1 [iid]
  (let [[a b] (split-mid iid)]
    (= a b)))

; Generate all substrings of the string from the start to the middle
; "abcdef" -> ["a", "ab", "abc"]
(defn all-substrs [iid]
  (let [mid (/ (count iid) 2)
        strs (for [n (range 1 (inc mid))] (subs iid 0 n))]
    strs))

; Given a substring, calculate how many times it would need to be
; repeated to match the length of the iid. If the repeated substring
; is the same as the iid and the substring is not the iid, the substring
; is repeated.
; "101010" "10" -> "10" * 3 -> "101010" == "101010"
; "101010" "1" -> "1" * 6 -> "111111" != "101010"
(defn is-repeated [iid substr]
  (let [lenId (count iid)
        lenSubstr (count substr)
        repeated (/ lenId lenSubstr)
        generated (for [_ (range 0 repeated)] substr)
        joined (apply str generated)]
    (and (not= lenId lenSubstr) (= iid joined))))

; Check if the id is comprised solely of repeated sequences of numbers.
; "555" -> "5" * 3, true
; "556" -> false
(defn is-invalid2 [iid]
  (let [substrs (all-substrs iid)]
    (not (not-any? identity (map #(is-repeated iid %) substrs)))))

; "93-97" -> [93, 94, 95, .., 97]
(defn all-ids [form]
  (let [[_          start end]
        (re-find #"(\d+)-(\d+)" form)
        start (Long/parseLong start)
        end (Long/parseLong end)
        all (for [id (range start (inc end))] (str id))]
    all))

; [false false true] -> true
; [false false false] -> false
(defn any-true [& fns]
  (fn [id] (not (not-any? identity ((apply juxt fns) id)))))

(defn solve [file & valid-fns]
  (let [idFilter #((apply any-true valid-fns) %)]
    (as-> file $
      (slurp $)
      (clojure.string/split $ #",")
      (mapcat all-ids $)
      (filter idFilter $)
      (map #(Long/parseLong %) $)
      (reduce + 0 $))))

(def part1 (solve "d2.txt" is-invalid1))
(def part2 (solve "d2.txt" is-invalid1 is-invalid2))
(println "Part 1: " part1)
(println "Part 2: " part2)
