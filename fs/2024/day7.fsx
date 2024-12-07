#load "AoCShared.fsx"

open AoCShared
open System

type D7 = { res: int64; nums: int64 list}

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

//gen all combos of the operators from 2 to <some upper bound> as a table,
//then just select the corresponding list from the table for each input nums
//evaluate each input against the set of lists, if any = the result it's valid

//only problem is I'm not good at math or DSA :D
//and every single function I found online to generate the permutations didn't work :D
//but it's okay because python has itertools.permutations :D
