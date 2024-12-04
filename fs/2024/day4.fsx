#load "AoCShared.fsx"

open AoCShared
open System

let input = rf "day4.txt" |> toCharGrid
let input2 = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX" |> toCharGrid

type Grid = char array array

let dims (grid: Grid) =
    let width = Array.length grid[0]
    let height = Array.length grid
    width, height

type Dir =
| N
| S
| E
| W
| NE
| NW
| SE
| SW

let dirs = [N; S; E; W; NE; NW; SE; SW]

let coordsFor (x, y) = function
    | N -> [x, y; x, y - 1; x, y - 2; x, y - 3]
    | S -> [x, y; x, y + 1; x, y + 2; x, y + 3]
    | E -> [x, y; x + 1, y; x + 2, y; x + 3, y]
    | W -> [x, y; x - 1, y; x - 2, y; x - 3, y]
    | NE -> [x, y; x + 1, y - 1; x + 2, y - 2; x + 3, y - 3]
    | NW -> [x, y; x - 1, y - 1; x - 2, y - 2; x - 3, y - 3]
    | SE -> [x, y; x + 1, y + 1; x + 2, y + 2; x + 3, y + 3]
    | SW -> [x, y; x - 1, y + 1; x - 2, y + 2; x - 3, y + 3]

let inBounds grid (x, y) = 
    let (w, h) = dims grid
    x >= 0 && y >= 0 && x < w && y < h

let charAt (grid: Grid) (x, y) = grid[y][x]

let isXmas dir (x, y) grid =
    let maybeXmas = 
        coordsFor (x, y) dir
        |> List.filter (inBounds grid)
        |> List.map (charAt grid)

    // printfn "%A" (dir, (x, y), maybeXmas)
    match maybeXmas with
    | ['X';'M';'A';'S'] -> true
    | _ -> false

let countXmas grid (x, y) =
    dirs
    |> List.filter (fun dir -> isXmas dir (x, y) grid)
    |> List.length

let allXes grid =
    let (w, h) = dims grid
    
    List.allPairs [0..w - 1] [0..h - 1]
    |> List.filter (charAt grid >> (=)'X')

let solveP1 grid =
    allXes grid
    |> List.map (countXmas grid)
    |> List.sum

let p1 = solveP1 input
printfn $"%d{p1}"

// let (w, h) = dims input2
// List.allPairs [0..w - 1] [0..h - 1]
// |> List.map (fun coord -> coord, countXmas input2 coord)
// |> List.iter (printfn "%A")
