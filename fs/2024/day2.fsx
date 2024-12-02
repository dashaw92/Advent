#load "AoCShared.fsx"

open AoCShared
open System

let cleanNums (str: string) = 
    str.Split(' ') 
    |> Array.map _.Trim() 
    |> Array.filter (String.IsNullOrEmpty >> not)
    |> Array.map int
    |> List.ofArray

let toDay2 =
    List.ofArray
    >> List.map cleanNums
    >> List.filter (fun a -> a.Length > 0)

let input =
    rf "day2.txt"
    |> lines
    |> toDay2

let input2 = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9" |> lines |> toDay2

let normalize =
    function
    | a :: b :: _ when a > b -> List.rev
    | _ -> id

let solve nums =
    nums
    |> normalize nums
    |> List.pairwise
    |> List.forall (fun (a, b) -> a < b && abs(a - b) <= 3)

//Doesn't work because we love weekend puzzles
let solve2 nums =
    let normal = normalize nums <| nums
    let filtered =
        normal
        |> List.windowed 2 
        |> List.filter (fun n -> n[0] < n[1] && abs(n[0] - n[1]) <= 3)
    printfn $"%A{filtered} - %A{normal}"
    (List.length normal) - (List.length filtered) < 2

let p1 = 
    input2
    |> List.map solve
    |> List.filter id
    |> List.length

    
let p2 = 
    input2
    |> List.map solve2
    |> List.filter id
    |> List.length

printfn $"%i{p1}"
printfn $"%i{p2}"
