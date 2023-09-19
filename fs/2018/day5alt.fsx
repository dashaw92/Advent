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
                
let p1: int = solve input [] |> List.length
printfn $"%A{p1}"