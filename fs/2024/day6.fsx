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
    |> List.filter (fun pos -> atGrid grid pos = '#')

let updateSteps steps pos dir =
    Map.change pos (function
    | Some prev -> Some (Set.add dir prev)
    | None -> Some (Set.add dir Set.empty)) steps
    
let seenBefore steps pos dir =
    match Map.tryFind pos steps with
    | Some prev -> Set.contains dir prev
    | None -> false

// let seenBefore steps pos dir = 
//     defaultArg (Option.map (Set.contains dir) (Map.tryFind pos steps)) false
    
let rec turn walls (x, y) dir =
    let (dx, dy) = delta dir
    if List.contains (x + dx, y + dy) walls then
        turn walls (x, y) (rot dir)
    else
        dir
    
    
let rec move steps walls dims dir (x, y) =
    let nextSteps = updateSteps steps (x, y) dir
    let nd = turn walls (x, y) dir
    let (dx, dy) = delta nd
    let (nx, ny) = (x + dx, y + dy)

    if offGrid dims (nx, ny) then
        nextSteps
    else
        move nextSteps walls dims nd (nx, ny)

let rec isLoop steps walls dims dir (x, y) =
    if seenBefore steps (x, y) dir then
        true
    else
        let nextSteps = updateSteps steps (x, y) dir
        let nd = turn walls (x, y) dir
        let (dx, dy) = delta nd
        let (nx, ny) = (x + dx, y + dy)
        if offGrid dims (nx, ny) then
            false
        else
            isLoop nextSteps walls dims nd (nx, ny)

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
    out
    |> Map.count

let solveP2 grid =
    let (w, h) = dims grid
    let guard = getGuard grid
    let walls = toWalls grid
    
    let generateAndRun grid (x, y) =
        if guard = (x, y) || List.contains (x, y) walls then
            false
        else
            let nWalls = (x, y) :: walls
            isLoop Map.empty nWalls (w, h) N guard

    let route = move Map.empty walls (w, h) N guard

    route
    |> Map.keys
    |> Array.ofSeq
    |> Array.Parallel.map (generateAndRun grid)
    |> Array.filter id
    |> Array.length

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
printfn "%d" <| solveP2 input
