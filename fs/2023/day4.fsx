open System
open System.Text.RegularExpressions

type Card = { Id: int; Matches: int; Points: int }

let removeExcessSpaces (line: string) = Regex.Replace (line, " {2,}", " ")

let toNums (numbers: string) =
    numbers.Split " "
    |> Array.map (fun str -> str.Trim ())
    |> Array.filter (not << String.IsNullOrEmpty)
    |> Array.map int
    |> List.ofArray    

let points winning numbers =
    let power = 
        numbers
        |> List.filter (fun number -> List.contains number winning)
        |> List.length

    match power with
    | x when x > 0 -> x, pown 2 (x - 1)
    | _ -> 0, 0

let parseCard (card: string) =
    let card = removeExcessSpaces card
    let sp = card.Split " | "
    let id = ((sp[0].Split " ").[1].Split ":").[0] |> int
    let winning = toNums (sp[0].Split ": ").[1]
    let numbers = toNums sp[1]

    let (matches, points) = points winning numbers
    { Id = id; Matches = matches; Points = points }

let getOr0 m k = defaultArg (Map.tryFind k m) 0
let insertOrChange m (vFn: int -> int) k =
    let v = getOr0 m k
    Map.add k (vFn v) m

let merge (m1: Map<'a, int>) (m2: Map<'a, int>) =
    let m1k = m1.Keys |> List.ofSeq
    let m2k = m2.Keys |> List.ofSeq

    List.concat (seq [ m1k; m2k ])
    |> List.map (fun cardId -> cardId, max (getOr0 m1 cardId) (getOr0 m2 cardId))
    |> Map

let processP2 cards =
    let reduce acc card =
        let current = card.Matches
        let amount = getOr0 acc card.Id
        if current <> 0 then
            [card.Id + 1 .. card.Id + current]
            |> List.map (insertOrChange acc ((+) amount))
            |> List.reduce merge
        else
            acc

    let originalMap = List.map (fun x -> x.Id, 1) cards |> Map
    List.fold reduce originalMap cards
    |> Map.values
    |> Seq.sum

let solveP1 = List.map parseCard >> List.map _.Points >> List.sum
let solveP2 = List.map parseCard >> processP2

let input =
    IO.File.ReadLines($"{__SOURCE_DIRECTORY__}/day4.txt") |> List.ofSeq

let p1 = solveP1 input
let p2 = solveP2 input