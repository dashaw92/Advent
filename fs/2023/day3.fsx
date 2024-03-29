#load "AoCShared.fsx"

open AoCShared
open System

type Schem =
    { Grid: char array
      Width: int
      Height: int }

type Part = { X: int; Y: int; Value: int }

let parseSchem (str: string) =
    let lines = str.Split '\n'

    let width = lines[0].Length
    let height = lines.Length
    let grid = (str.Replace("\n", "")).ToCharArray()

    { Grid = grid
      Width = width
      Height = height }

let isSymbol ch = ch <> '.' && not <| Char.IsAsciiDigit ch

let isGear = (=) '*'
let isPart = Char.IsAsciiDigit

let findSymbols filter schem =
    schem.Grid
    |> Array.indexed
    |> List.ofArray
    |> List.filter (snd >> filter)
    |> List.map (fun (idx, _) -> idx % schem.Width, idx / schem.Width)

let neighbors (x, y) =
    [ x - 1, y - 1
      x - 1, y
      x - 1, y + 1
      x, y - 1
      x, y + 1
      x + 1, y - 1
      x + 1, y
      x + 1, y + 1 ]

let inBounds schem (x, y) =
    x >= 0 && x < schem.Width && y >= 0 && y < schem.Height

let idx schem (x, y) = schem.Grid[y * schem.Width + x]

let partAt schem (x, y) =
    let rec findIdx dirFn (x, y) =
        match x with
        | x when (not <| inBounds schem (dirFn x, y)) -> x
        | x when (not <| isPart (idx schem (dirFn x, y))) -> x
        | _ -> findIdx dirFn (dirFn x, y)

    let startIdx = findIdx ((+) -1) (x, y)
    let endIdx = findIdx ((+) 1) (x, y)

    let number =
        schem.Grid[(y * schem.Width + startIdx) .. (y * schem.Width + endIdx)]
        |> String.Concat
        |> int

    { X = startIdx; Y = y; Value = number }

let collectParts filter schem =
    findSymbols filter schem
    |> List.map neighbors
    |> List.map (List.filter (inBounds schem))
    |> List.map (List.filter (idx schem >> isPart))

let findParts schem =
    collectParts isSymbol schem
    |> List.fold (List.append) []
    |> List.map (partAt schem)
    |> List.distinctBy (fun part -> (part.X, part.Y))

let findGears schem =
    collectParts isGear schem
    |> List.map (List.map (partAt schem))
    |> List.map (List.distinctBy (fun part -> (part.X, part.Y)))
    |> List.filter (List.length >> (=) 2)

let solveP1 = 
    parseSchem 
    >> findParts 
    >> List.map _.Value 
    >> List.sum

let solveP2 =
    parseSchem
    >> findGears
    >> List.map (List.map _.Value)
    >> List.map (List.reduce ( * ))
    >> List.sum

let input = rf "day3.txt"

let p1 = solveP1 input
let p2 = solveP2 input
