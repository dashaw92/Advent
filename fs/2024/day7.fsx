#load "AoCShared.fsx"

open AoCShared
open System

type D7 = { res: int64; nums: int64 list}
type Op = int64 -> int64 -> int64

let toDay7 (st: string) =
    let split = st.Split ':'
    let res = int64 <| split[0]
    let nums =
        (despace split[1]).Split ' '
        |> Array.filter (_.Length >> (<>)0)
        |> Array.map int64
        |> List.ofArray
    { res = res; nums = nums }

let concat (a: int64) (b: int64) = int64 <| sprintf $"%d{a}%d{b}"
let opers1 = [ (+); (*) ]
let opers2 = [ (+); (*); concat ]
let input = rf "day7.txt" |> lines |> List.ofArray |> List.map toDay7

let genAll (ops: 'a list) n =
    let toBaseStr b n =
        if n = 0 then
            "0"
        else
            let rec aux buf i =
                if i <= 0 then
                    buf
                else
                    let bufN = sprintf "%d%s" (i % b) buf
                    aux bufN (i / b)
            let out = aux "" n
            out
        

    let padStr len (s: string) =
        if s.Length >= len then
            s
        else
            sprintf "%s%s" (String.replicate (len - s.Length) "0") s

    let toOps (num: string) =
        num.ToCharArray()
        |> Array.map (fun c -> ops[int c - (int '0')])
        |> List.ofArray
    
    let m = List.length ops
    let maxlen = (toBaseStr m ((pown m n) - 1)).Length
    [0..(pown m n) - 1]
    |> List.map (toBaseStr m)
    |> List.map (padStr maxlen)
    |> List.map toOps

let genTable ops =
    [1..12]
    |> List.map (fun i -> i, genAll ops i)
    |> Map.ofList

let table1 = genTable opers1
let table2 = genTable opers2

let getOpers ops n = Option.get <| Map.tryFind n ops

let rec collapse nums ops =
    match nums with
    | [x] -> x
    | x :: n :: ns ->
        let res = (List.head ops) x n
        collapse (res :: ns) (List.tail ops)
    | [] -> failwith "Bad input"

let solveSingle ops d7 =
    getOpers ops (d7.nums.Length - 1)
    |> List.map (collapse d7.nums)
    |> List.exists (fun solved -> solved = d7.res)

let solve table = List.filter (solveSingle table) >> List.map _.res >> List.sum

let solveP1 = solve table1
let solveP2 = solve table2

let p1 = solveP1 input
printfn $"%d{p1}"

let p2 = solveP2 input
printfn $"%d{p2}"
