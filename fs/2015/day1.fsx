(*
    Author: dashaw92
    8/25/2023

    Advent of Code 2015 Day 1
*)

//Read the input and flatten it to a single string
let input = 
    System.IO.File.ReadLines($"{__SOURCE_DIRECTORY__}/day1.txt") 
    |> Seq.toList
    |> String.concat " "

let stairVal stair =
    match stair with
    | '(' -> 1
    | ')' -> -1
    | _ -> failwith "Bad input"

//Parsed form of the input, where all "stairs" are converted to int deltas
//to be applied to the current stair
let mappedInput =
    input |> Seq.map stairVal |> List.ofSeq

//Original part1. Not needed
// let part1 =
//     mappedInput
//     |> Seq.sum

//My strategy for solving this is simple. For every index i of the input,
//get the sum of the sublist of all indices from start to i set the current
//value in the list to the sum.
//By doing this, we get an output list where each element corresponds to Santa's
//stair level at that index

(* 
* Update 9/9/2023: Imagine my surprise when I find out that there's a function that does EXACTLY this :)
* let sums input =
*     let buildSums idx _ =
*         input
*         |> List.take (idx + 1)
*         |> List.sum
*     input
*     |> List.mapi buildSums
* 
* //Finally, get the solutions for both parts
* let solvedInput = 
*     mappedInput
*     |> sums
*)
let solvedInput = mappedInput |> List.scan (+) 0

//P2 cares about when we first reach level -1
let part2Pred elem = elem = -1

//The last element is Santa's final position
let part1 = solvedInput |> List.last
let part2 = solvedInput |> List.findIndex part2Pred

printfn $"Part 1: {part1} | Part 2: {part2}"