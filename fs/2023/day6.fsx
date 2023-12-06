#load "AoCShared.fsx"

open AoCShared
open System

type Race = { Time: int64; Record: int64 }
let race (time, record) = { Time = time; Record = record }

let parseInput input =
    (despace input).Split '\n'
    |> Array.map (fun line -> (line.Split ' ')[1..])
    |> Array.map (Array.map int64)
    |> (fun arrays -> Array.zip arrays[0] arrays[1] |> Array.map race)

let combineRaces input =
    (despace input).Split '\n'
    |> Array.map (fun line -> (line.Split ' ')[1..])
    |> Array.map (String.Concat)
    |> Array.map int64
    |> (fun values -> { Time = values[0]; Record = values[1] })
    |> Array.singleton

let allMethods race =
    let min = 1L
    let max = race.Time - 1L
    seq {
        for i in min..max do
            let speed = i
            yield i * (race.Time - i)
    } |> Seq.filter (fun method -> method > race.Record)

let solve parser =
    parser
    >> Array.map allMethods
    >> Array.map (Seq.length)
    >> Array.reduce ( * )

let solveP1 = solve parseInput
let solveP2 = solve combineRaces

let input = rf "day6.txt"

let p1 = solveP1 input