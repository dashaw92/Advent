#load "AoCShared.fsx"

open AoCShared
open System

type Grid = char array array

let input = rf "day6.txt" |> toCharGrid

let getGuard grid =
    let y = Array.findIndex (Array.contains '^') grid
    let x = Array.findIndex ((=)'^') grid[y]

    (x, y)

type D = | N | E | S | W
let delta = function
| N -> (0, -1)
| E -> (1, 0)
| S -> (0, 1)
| W -> (-1, 0)

let rot = function
| N -> E
| E -> S
| S -> W
| W -> N

let dims (grid: Grid) =
    let w = Array.length grid[0]
    let h = Array.length grid
    w, h
    
let offGrid dims (x, y) =
    let (w, h) = dims
    x < 0 || y < 0 || x >= w || y >= h

let atGrid (grid: Grid) (x, y) = grid[y][x]

let toWalls grid =
    let (w, h) = dims grid
    List.allPairs [0..w - 1] [0..h - 1]
    // |> List.iter (printfn "%A")
    |> List.filter (fun pos -> atGrid grid pos = '#')
    // []

let updateSteps steps pos =
    Map.change pos (function
    | Some count -> Some (count + 1)
    | None -> Some 1) steps
    
let rec move steps walls dims dir (x, y) =
    let nextSteps = updateSteps steps (x, y)
    let (dx, dy) = delta dir
    let (nx, ny) = (x + dx, y + dy)
    if List.contains (nx, ny) walls then
        let nd = rot dir
        let (nx, ny) = delta nd
        move nextSteps walls dims nd (x + nx, y + ny)
    else
        if offGrid dims (nx, ny) then
            nextSteps
        else
            move nextSteps walls dims dir (nx, ny)

let printIt walls steps (w, h) =
    [0..h - 1]
    |> List.map (fun y ->
        [0..w - 1]
        |> List.map (fun x ->
            match Map.tryFind (x, y) steps with
            | Some _ -> "X"
            | None ->
                if List.contains (x, y) walls then
                    "#"
                else
                    "."
        )
    )
    |> List.map (String.concat "")
    |> String.concat "\n"
    |> printfn "%s"

let solveP1 grid =
    let guard = getGuard grid
    let walls = toWalls grid
    let gdims = dims grid

    let out = move Map.empty walls gdims N guard
    printIt walls out gdims
    out
    |> Map.filter (fun _ count -> count >= 1)
    |> Map.count
    // |> printfn "%A"
    // 0

let ex = toCharGrid "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."

printfn "%d" <| solveP1 input
