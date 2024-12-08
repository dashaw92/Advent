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

let isXmas dir (x, y) (grid: Grid) =
    let maybeXmas = 
        coordsFor (x, y) dir
        |> List.filter (inGrid (dims grid))
        |> List.map (atGrid grid)

    // printfn "%A" (dir, (x, y), maybeXmas)
    match maybeXmas with
    | ['X';'M';'A';'S'] -> true
    | _ -> false

let isMas grid (x, y) =
    let nToS = [x - 1, y - 1; x, y; x + 1, y + 1]
    let eToW = [x - 1, y + 1; x, y; x + 1, y - 1]

    let getMas coords =
        coords
        |> List.filter (inGrid (dims grid))
        |> List.map (atGrid grid)

    let maybeMas1 = getMas nToS
    let maybeMas2 = getMas eToW

    let check1 = maybeMas1 = ['M'; 'A'; 'S'] || maybeMas1 = ['S'; 'A'; 'M']
    let check2 = maybeMas2 = ['M'; 'A'; 'S'] || maybeMas2 = ['S'; 'A'; 'M']
    check1 && check2

let countXmas grid (x, y) =
    dirs
    |> List.filter (fun dir -> isXmas dir (x, y) grid)
    |> List.length

let findAll grid ch =
    let (w, h) = dims grid
    
    List.allPairs [0..w - 1] [0..h - 1]
    |> List.filter (atGrid grid >> (=)ch)

let solveP1 grid =
    findAll grid 'X'
    |> List.map (countXmas grid)
    |> List.sum

let p1 = solveP1 input
printfn $"%d{p1}"


let solveP2 grid =
    findAll grid 'A'
    |> List.map (isMas grid)
    |> List.filter id
    |> List.length

let p2 = solveP2 input
printfn $"%d{p2}"
// let (w, h) = dims input2
// List.allPairs [0..w - 1] [0..h - 1]
// |> List.map (fun coord -> coord, countXmas input2 coord)
// |> List.iter (printfn "%A")
