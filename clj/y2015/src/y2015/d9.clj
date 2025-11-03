(ns y2015.d9
  (:require [clojure.string :as str]))

(def ex (str/split-lines (slurp "resources/day9ex.txt")))
;; (def input (str/split-lines (slurp "resources/day9.txt")))

(defn parse-route [line]
  (let [groups (re-matches #"(\w+) to (\w+) = (\d+)" line)
        start (nth groups 1)
        end (nth groups 2)
        dist (Integer/parseInt (nth groups 3))]
    {:start start :dest end :dist dist}))


(defn collapse-routes [m curr]
  (let [{:keys [start dest dist]} curr
        destEntry {:dest dest :dist dist}]
    (update m start #(conj % destEntry))))

(def routes (map parse-route ex))
(reduce collapse-routes {} routes)
