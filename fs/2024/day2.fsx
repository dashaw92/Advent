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

let solve2 nums =
    //Generates versions of the list with one element removed
    //from 0..n
    let len = List.length nums
    let rec brute lists i ns =
        match i with
        | i when i = len -> lists
        | _ -> brute ((List.removeAt i ns) :: lists) (i + 1) nums
    
    nums
    |> brute [] 0
    |> List.map solve
    |> List.exists id

let p1 = 
    input
    |> List.map solve
    |> List.filter id
    |> List.length

    
let p2 = 
    input
    |> List.map solve2
    |> List.filter id
    |> List.length

printfn $"%i{p1}"
printfn $"%i{p2}"
