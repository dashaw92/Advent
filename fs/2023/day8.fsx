#load "AoCShared.fsx"
open AoCShared
open System
open System.Text.RegularExpressions

type Node = { Left: string; Right: string }
type Input = { Instrs: char array; Nodes: Map<string, Node> }

let getNode input name = input.Nodes.TryFind name |> Option.get
let endWith (ch: char) (str: string) = str.EndsWith ch
let allNodesThat filter input = input.Nodes.Keys |> Seq.filter filter

let nextInstr (idx: int64) input currentNode =
    let instr = input.Instrs[(idx |> int) % input.Instrs.Length]
    match instr with
    | 'L' -> currentNode.Left
    | 'R' -> currentNode.Right
    | _ -> failwithf $"Bad instruction: {instr}"

let parse (input: string list) =
    let parseNode (input: string) =
        Regex.Matches (input, "[0-9A-Z]{3}")
        |> (fun matches -> (matches.Item 0).Value, { Left = (matches.Item 1).Value; Right = (matches.Item 2).Value})

    let instrs = input[0].ToCharArray ()
    let nodes = input[2..] |> List.map parseNode |> Map
    
    { Instrs = instrs; Nodes = nodes }

let rec findEndNode steps node input =
    let gn = getNode input
    let next = (gn >> nextInstr steps input) node
    if endWith 'Z' next then steps + 1L
    else findEndNode (steps + 1L) next input

let lcm_all =
    let rec gcd a b =
        match b with
        | 0L -> a
        | _ -> gcd b (a % b)
    
    let lcm a b = a * b / (gcd a b)
    Seq.reduce lcm

let traverse2 input =
    allNodesThat (endWith 'A') input 
    |> Seq.map (fun node -> findEndNode 0 node input)
    |> lcm_all

let solveP1 = parse >> findEndNode 0 "AAA"
let solveP2 = parse >> traverse2

let input = (rf "day8.txt").Split '\n' |> List.ofArray

let p1 = solveP1 input
let p2 = solveP2 input