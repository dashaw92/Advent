#load "AoCShared.fsx"

open AoCShared
open System

type Grid = char array array

let input = rf "day8.txt" |> toCharGrid
let ex = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............" |> toCharGrid

let antennas grid =
    let intoMap map (pos, ant) =
        Map.change ant (function
        | Some curr -> Some (pos :: curr)
        | None -> Some [pos]) map

    iterGrid grid
    |> List.filter (snd >> (<>)'.')
    |> List.fold intoMap Map.empty

let dist (a, b) (x, y) = (-1 * (x - a), -1 * (y - b))

let add (a, b) (x, y) = (a + x, b + y)

let getNodes grid (ant, locs) =
    List.allPairs locs locs
    |> List.filter (fun (a, b) -> a <> b)
    |> List.map (fun (a, b) -> add a (dist a b))
    |> List.distinct

let antiNodes grid antennas =
    Map.toSeq antennas
    |> Seq.map (getNodes grid)
    |> Seq.reduce List.append
    |> List.distinct
    |> List.filter (inGrid (dims grid))
    |> List.length

let solveP1 g = antiNodes g (antennas g) 

printfn "%A" (solveP1 input)
