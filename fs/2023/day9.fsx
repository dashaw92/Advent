#load "AoCShared.fsx"
open AoCShared
open System

let lineToInts (str: string) = (str.Split ' ') |> Array.toList |> List.map int
let flip (a, b) = b, a

let findNextValue (ints: int list) =
    let rec getDeltas ints (acc: int list list) =
        let deltas = 
            List.pairwise ints
            |> List.map (flip >> (<||) (-))
        if List.forall ((=) 0) deltas then deltas :: acc
        else getDeltas deltas (deltas :: acc)

    let deltas = getDeltas ints [ints]
    deltas
    |> List.map List.last
    |> List.sum

let solveP1 = List.map findNextValue >> List.sum

let example =
    "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45".Split '\n' |> Array.toList |> List.map lineToInts

let input = (rf "day9.txt").Split '\n' |> Array.toList |> List.map lineToInts