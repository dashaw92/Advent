#load "AoCShared.fsx"

open AoCShared
open System

type Race = { Time: int64; Record: int64 }
let race (time, record) = { Time = time; Record = record }

let parseInput input =
    (despace input).Split '\n'
    |> Array.map (fun line -> (line.Split ' ')[1..] |> Array.map int64)
    |> (fun arrays -> Array.zip arrays[0] arrays[1] |> Array.map race)

let combineRaces input =
    (despace input).Split '\n'
    |> Array.map (fun line -> (line.Split ' ')[1..] |> String.Concat |> int64)
    |> (fun values -> values[0], values[1])
    |> race
    |> Array.singleton

let allMethods race =
    let min = 1L
    let max = race.Time - 1L
    seq [min .. max]
    |> Seq.map (fun i -> i * (race.Time - i))
    |> Seq.filter ((<) race.Record)

let solve parser =
    parser
    >> Array.map allMethods
    >> Array.map (Seq.length)
    >> Array.reduce ( * )

let solveP1 = solve parseInput
let solveP2 = solve combineRaces

let input = rf "day6.txt"

let p1 = solveP1 input
let p2 = solveP2 input