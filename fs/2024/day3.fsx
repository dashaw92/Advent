#load "AoCShared.fsx"

open AoCShared
open System
open System.Text.RegularExpressions

let cleanNums (str: string) = 
    str.Split(' ') 
    |> Array.map _.Trim() 
    |> Array.filter (String.IsNullOrEmpty >> not)
    |> Array.map int
    |> List.ofArray

let input = rf "day3.txt"
let input2 = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"

let re = Regex("mul\((\d+),(\d+)\)", RegexOptions.Compiled)

let allMatches (re: Regex) str =
    re.Matches(str)
    |> Seq.map (fun m -> int m.Groups[1].Value, int m.Groups[2].Value)

let p1 =
    allMatches re input
    |> Seq.map (fun (a, b) -> a * b)
    |> Seq.sum

printfn $"%A{p1}"
