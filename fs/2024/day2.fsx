#load "AoCShared.fsx"

open AoCShared
open System

let cleanNums (str: string) = 
    str.Split(' ') 
    |> Array.map _.Trim() 
    |> Array.filter (String.IsNullOrEmpty >> not)
    |> Array.map int

let toDay2 =
    List.ofArray
    >> List.map cleanNums
    >> List.filter (fun a -> a.Length > 0)

let input =
    rf "day2.txt"
    |> lines
    |> toDay2

// let input = "7 6 4 2 1
// 1 2 7 8 9
// 9 7 6 2 1
// 1 3 2 4 5
// 8 6 4 4 1
// 1 3 6 7 9" |> lines |> toDay2

type Report =
| Inc
| Dec
| Invalid

let safe reportType (a, b) =
    match reportType with
    | Inc when abs(a - b) < 4 -> a < b
    | Dec when abs(a - b) < 4 -> a > b
    | _ -> false

let isSafe (report: int array) =
    let pairs = Array.pairwise report
    let reportType = 
        match pairs[0] with
        | (a, b) when a < b -> Inc
        | (a, b) when a > b -> Dec
        | _ -> Invalid

    if reportType = Invalid then
        false
    else    
        Array.pairwise report
        |> Array.forall (safe reportType)

let p1 =
    input
    |> List.filter isSafe
    |> List.length

printfn $"%i{p1}"
