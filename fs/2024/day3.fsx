#load "AoCShared.fsx"

open AoCShared
open System
open System.Text.RegularExpressions

let input = rf "day3.txt"
let input2 = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
let input3 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"

let findMuls = Regex("mul\((\d+),(\d+)\)", RegexOptions.Compiled)
let findDos = Regex("do\(\)", RegexOptions.Compiled)
let findDonts = Regex("don't\(\)", RegexOptions.Compiled)

let allMuls str =
    findMuls.Matches(str)
    |> Seq.map (fun m -> m.Index, int m.Groups[1].Value, int m.Groups[2].Value)

let getIndexesOf (re: Regex) str =
    re.Matches(str)
    |> Seq.map (fun m -> m.Index)

let idxAndTotal input =
    allMuls input
    |> Seq.map (fun (idx, a, b) -> idx, a * b)

let p1 =
    idxAndTotal input
    |> Seq.map snd
    |> Seq.sum


printfn $"%A{p1}"

type P2 =
| Do of int
| Dont of int

let valueOf = function
    | Do x -> x
    | Dont x -> x

let toBool = function
    | Do _ -> true
    | Dont _ -> false
    
let allDosAndDonts input =
    let dos = 
        getIndexesOf findDos input 
        |> List.ofSeq |> List.map Do
    let donts = 
        getIndexesOf findDonts input 
        |> List.ofSeq |> List.map Dont
    
    List.append dos donts
    |> List.append [Do 0]
    |> List.sortByDescending valueOf

let shouldMul dosDonts idx =
    toBool <| List.find (fun flag -> idx > valueOf flag) dosDonts

let p2 =
    idxAndTotal input
    |> Seq.filter (fst >> (shouldMul <| allDosAndDonts input))
    |> Seq.map snd
    |> Seq.sum

printfn $"%A{p2}"
