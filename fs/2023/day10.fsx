#load "AoCShared.fsx"
open AoCShared
open System

type Dir = North | South | East | West
let pipes =
    [
        '|', [North; South]
        '-', [East; West]
        'L', [North; East]
        'J', [North; West]
        '7', [South; West]
        'F', [South; East]
        'S', [North; East; South; West]
    ] |> Map
let getPipe ch = Map.tryFind ch pipes |> Option.get
let neighbors (x, y) =
    [ x - 1, y
      x, y - 1
      x, y + 1
      x + 1, y ]
let relativeDir (x1, y1) (x2, y2) =
    match (x1 - x2, y1 - y2) with
    | -1, _ -> West
    | 1, _ -> East
    | _, -1 -> North
    | _, 1 -> South
    | _ -> failwith $"? {x1}, {y1} - {x2}, {y2}"
let invert = function
    | North -> South
    | South -> North
    | East -> West
    | West -> East

type Pos = int * int
type Maze = 
    { Width: int; Height: int; Start: Pos; Chars: char array }
    
    member maze.toIdx (x, y) = y * maze.Width + x
    member maze.fromIdx idx = idx % maze.Width, idx / maze.Width
    member maze.pipeAt pos = maze.Chars[maze.toIdx pos]
    member maze.inBounds (x, y) = x >= 0 && x < maze.Width && y >= 0 && y < maze.Height
    member maze.canConnect p1 p2 =
        if not (maze.inBounds p1) || not (maze.inBounds p2) then false else
        if maze.pipeAt p1 = '.' || maze.pipeAt p2 = '.' then false else

        let pipe1 = getPipe <| maze.pipeAt p1
        let pipe2 = getPipe <| maze.pipeAt p2

        let dir = relativeDir p1 p2
        List.contains (invert dir) pipe1 && List.contains dir pipe2

type Conns = Map<Pos, Pos list>

let makeMaze (input: string) =
    let lines = input.Split '\n'

    let width = lines[0].Length
    let height = lines.Length

    let startY = lines |> Array.findIndex (fun line -> line.Contains 'S')
    let startX = lines[startY] |> (fun line -> line.IndexOf 'S')

    let chars = lines |> Array.map (fun line -> line.ToCharArray ()) |> Array.reduce Array.append
    { Width = width; Height = height; Start = startX, startY; Chars = chars }


let buildPipes (maze: Maze) =
    let rec aux conns pos =
        let conns = pos :: conns
        let nextTo = 
            neighbors pos 
            |> List.filter (fun pos -> not <| List.contains pos conns)
            |> List.filter maze.inBounds 
            |> List.filter (maze.canConnect pos)
        if List.isEmpty nextTo then conns else
        aux conns (List.head nextTo)
    
    aux [] maze.Start

let solveP1 = buildPipes >> List.length >> (fun len -> len / 2)

let input = rf "day10.txt" |> makeMaze
let example =
    ".....
.S-7.
.|.|.
.L-J.
....." |> makeMaze

let example2 =
    "..F7.
.FJ|.
SJ.L7
|F--J
LJ..." |> makeMaze