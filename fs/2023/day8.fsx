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

let traverse start goal input =
    let gn = getNode input

    let rec aux steps start goal =
        match start with
        | isGoal when isGoal = goal -> steps
        | _ ->
            let next = gn <| nextInstr steps input start
            aux (steps + 1L) next goal
    aux 0L (gn start) (gn goal)

let lcm_all =
    let rec gcd a b =
        match b with
        | 0L -> a
        | _ -> gcd b (a % b)
    
    let lcm a b = a * b / (gcd a b)

    List.reduce lcm

let traverse2 input =
    let allStarts = allNodesThat (endWith 'A') input |> List.ofSeq
    let gn = getNode input

    let rec findEndNode steps node =
        let next = (gn >> nextInstr steps input) node
        if endWith 'Z' next then next
        else findEndNode (steps + 1L) next

    allStarts
    |> List.map (fun node -> node, findEndNode 0 node)
    |> List.map (fun (nodeA, nodeB) -> traverse nodeA nodeB input)
    |> lcm_all

let solveP1 = parse >> traverse "AAA" "ZZZ"
let solveP2 = parse >> traverse2

let input = (rf "day8.txt").Split '\n' |> List.ofArray