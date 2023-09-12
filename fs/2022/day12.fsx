type P = int * int
type GNode = char * P * (P list)
type Graph = GNode list

type PreGraph = char array array

let exampleInput = 
    [ "Sabqponm"
      "abcryxxl"
      "accszExk"
      "acctuvwj"
      "abdefghi" ] |> String.concat "\n"

let convertInput (input: string) =
    input.Split '\n'
    |> Array.map Seq.toArray

let len (arr: array<'a>) = arr.Length

let idx ((x, y): P) (pg: PreGraph) = 
    pg.[y].[x]

let neighbors ((x, y): P) =
    [
        (x + 1, y)
        (x - 1, y)
        (x, y + 1)
        (x, y - 1)
    ]

let toHeight ch = 
    match ch with
    | 'S' -> 0  //Start
    | 'E' -> 25 //End
    | _ -> (ch - 'a') |> int

let validStep (a: char) (b: char) = 
    let aHeight = toHeight a
    let bHeight = toHeight b
    aHeight = bHeight - 1 || aHeight >= bHeight

let possibleSteps (pg: PreGraph) (p: P): GNode =
    let inBounds ((x, y): P) =
        let w = len pg.[0]
        let h = len pg
        x >= 0 && x < w && y >= 0 && y < h

    let validNeighbors: P list = 
        neighbors p
        |> Seq.filter inBounds
        |> Seq.filter (fun np -> validStep (idx p pg) (idx np pg))
        |> Seq.toList
    (idx p pg, p, validNeighbors)

let graphOf (input: string): Graph =
    let pg: PreGraph = input |> convertInput
    
    let w = len pg.[0]
    let h = len pg
    let allPoints: P seq = 
        seq {
            for x in 0..w - 1 do
            for y in 0..h - 1 do
            yield (x, y)
        }
    
    allPoints
    |> Seq.map (possibleSteps pg)
    |> Seq.toList