let toUpper = System.Char.ToUpper

let input = 
    System.IO.File.ReadLines($"{__SOURCE_DIRECTORY__}/day5.txt") 
    |> Seq.toList
    |> String.concat ""
    |> Seq.toList

let reacts a b = a <> b && (toUpper a = b || toUpper b = a)

//This is an alternate solution to day 5 that uses a stack based approach
//instead of... whatever that first attempt is...
//It's a lot faster because it builds lists from the front, as opposed to
//tacking on elements at the end. Once the input is fully consumed,
//the stack is the solution.
let rec solve (input: char list) (stack: char list): char list =
    match input with
    | [] -> stack //No need to rev here. The length is the only part that matters
    | head :: tail ->
        match stack with
        | [] -> solve tail [head]
        | stackHead :: stackTail ->
            if reacts stackHead head then
                //"Pop" off the stackHead
                solve tail stackTail
            else
                //No reaction, "push" head onto the full stack (which includes stackHead still)
                solve tail (head :: stack)
           
//My favorite implementation of this puzzle so far. Uses fold to simplify the logic, but is
//the same logic as above.
let foldSolve input =
    //stack: fold accumulator
    //a: next element in the input list
    //For an empty accumulator, just return a in a singleton list
    //Otherwise, check the reaction. For a reaction, return just the tail. Otherwise,
    //return a cons stack
    let foldReact stack a =
        match stack with
        | [] -> [a]
        | b :: tail ->
            if reacts a b then
                tail
            else
                a :: stack
    input |> List.fold foldReact []

let p1: int = foldSolve input |> List.length
printfn $"%A{p1}"

//Haskell for Imperative Programmers #11 - Folding Exercises
//(I wanted to fully understand folding before implementing foldSolve)
module Folding =
    let rev = List.fold :: []
    let prefixes<'a> = List.fold (fun acc x -> (x :: List.head acc) :: acc) [[]]
    let prefixesB xs =
        let appendEl el xss = xss |> List.map (fun xs -> el :: xs)
        let folder x acc = [x] :: appendEl x acc
        List.foldBack folder xs []