open System
open System.Text.RegularExpressions

let despace (line: string) = Regex.Replace (line, " {2,}", " ")

let rf path = IO.File.ReadLines($"{__SOURCE_DIRECTORY__}/inputs/{path}") |> String.concat "\n"

let lines (str: string) = str.Split('\n')

type Grid = char array array
let toCharGrid: string -> Grid = lines >> Array.map _.ToCharArray()

let dims (grid: Grid) =
    let w = Array.length grid[0]
    let h = Array.length grid
    w, h
    
let offGrid dims (x, y) =
    let (w, h) = dims
    x < 0 || y < 0 || x >= w || y >= h

let inGrid dims (x, y) = not <| offGrid dims (x, y)

let atGrid (grid: Grid) (x, y) = grid[y][x]
