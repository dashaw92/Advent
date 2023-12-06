#load "AoCShared.fsx"

open AoCShared
open System

type Race = { Time: int; Record: int }
let race (time, record) = { Time = time; Record = record }

let parseInput input =
    (despace input).Split '\n'
    |> Array.map (fun line -> (line.Split ' ')[1..])
    |> Array.map (Array.map int)
    |> (fun arrays -> Array.zip arrays[0] arrays[1] |> Array.map race)

let allMethods race =
    let min = 1
    let max = race.Time - 1
    seq {
        for i in min..max do
            let speed = i
            yield i * (race.Time - i)
    } |> Seq.filter (fun method -> method > race.Record)

let solveP1 =
    parseInput
    >> Array.map allMethods
    >> Array.map (Seq.length)
    >> Array.reduce ( * )

let input = rf "day6.txt"
let example =
    "Time:      7  15   30
Distance:  9  40  200"

let p1 = solveP1 input