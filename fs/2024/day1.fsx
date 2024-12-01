#load "AoCShared.fsx"

open AoCShared
open System

let input = (rf "day1.txt").Split '\n' |> List.ofArray

//"a   b" -> (a, b)
let splitNum (a: string): int64 * int64 =
    let nums = (despace a).Split (' ')
    (int64 nums[0], int64 nums[1])

//(fold fn) Collapse an array of tuples to a tuple of lists
let appendNums (a, b) (aL, bL) =
    (aL :: a, bL :: b)

let sort (aL, bL) =
    (List.sort aL, List.sort bL)

let (aL, bL) = 
    input
    |> List.map splitNum
    |> List.fold appendNums ([], [])
    |> sort

let diffNum a b = abs a - b

let diffs = List.map2 diffNum aL bL
let p1 = List.sum diffs

printfn $"Part 1: %d{p1}"

let timesSeen num l =
    l 
    |> List.filter ((=) num) 
    |> List.length
    |> int64

let scoreNum bL a =
    let score = timesSeen a bL
    a * score

let scores = List.map (scoreNum bL) aL
let p2 = List.sum scores

printfn $"Part 2: %d{p2}"
