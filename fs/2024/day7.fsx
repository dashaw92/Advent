#load "AoCShared.fsx"

open AoCShared
open System

type D7 = { res: int64; nums: int64 list}
type Op = int -> int -> int

let toDay7 (st: string) =
    let split = st.Split ':'
    let res = int64 <| split[0]
    let nums =
        (despace split[1]).Split ' '
        |> Array.filter (_.Length >> (<>)0)
        |> Array.map int64
        |> List.ofArray
    { res = res; nums = nums }

let opers = [ (+); (*) ]
let input = rf "day7.txt" |> lines |> List.ofArray |> List.map toDay7

let genAll (ops: 'a list) n =
    let toOps num =
        [0..n - 1]
        |> List.map (fun idx -> (num >>> idx) &&& 1)
        |> List.map (function
        | 0 -> ops[0]
        | 1 -> ops[1]
        | _ -> failwith "Uh?")

    let m = List.length ops
    [0..(pown m n) - 1]
    |> List.map toOps

let table =
    [2..12]
    |> List.map (fun i -> i, genAll opers i)
    |> Map.ofList

let getOpers n =
    Option.get <| Map.tryFind n table

let rec collapse nums ops =
    match nums with
    | [x] -> x
    | x :: n :: ns ->
        let res = (List.head ops) x n
        collapse (res :: ns) (List.tail ops)
    | [] -> failwith "Bad input"

let solveSingle d7 =
    let ops = getOpers (d7.nums.Length - 1)
    ops
    |> List.map (collapse d7.nums)
    |> List.exists (fun solved -> solved = d7.res)

let solveP1 = List.filter solveSingle >> List.map _.res >> List.sum

let p1 = solveP1 input
printfn $"%d{p1}"
