#load "AoCShared.fsx"
open AoCShared
open System
open System.Text.RegularExpressions

type Node = { Left: string; Right: string }
type Input = { Instrs: char array; Nodes: Map<string, Node> }

let getNode input name = input.Nodes.TryFind name |> Option.get

let nextInstr idx input currentNode =
    let instr = input.Instrs[idx % input.Instrs.Length]
    match instr with
    | 'L' -> currentNode.Left
    | 'R' -> currentNode.Right
    | _ -> failwithf $"Bad instruction: {instr}"

let parse (input: string list) =
    let parseNode (input: string) =
        Regex.Matches (input, "[A-Z]{3}")
        |> (fun matches -> (matches.Item 0).Value, { Left = (matches.Item 1).Value; Right = (matches.Item 2).Value})

    let instrs = input[0].ToCharArray ()
    let nodes = input[2..] |> List.map parseNode |> Map
    
    { Instrs = instrs; Nodes = nodes }

let traverse start goal input =
    let gn = getNode input

    let rec aux steps start goal =
        match start with
        | isGoal when isGoal = goal -> steps
        | _ ->
            let next = gn <| nextInstr steps input start
            aux (steps + 1) next goal
    aux 0 (gn start) (gn goal)

let solveP1 =
    parse
    >> traverse "AAA" "ZZZ"

let example2Steps =
    "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)".Split '\n' |> List.ofArray

let example6Steps =
    "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)".Split '\n' |> List.ofArray
let input = (rf "day8.txt").Split '\n' |> List.ofArray