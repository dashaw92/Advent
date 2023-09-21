let toUpper = System.Char.ToUpper

let input = 
    System.IO.File.ReadLines($"{__SOURCE_DIRECTORY__}/day5.txt") 
    |> Seq.toList
    |> String.concat ""

let reacts a b = a <> b && (toUpper a = b || toUpper b = a)
            
let rec next = function
    | [] -> []
    | [a] -> [a]
    | a :: b :: tail when reacts a b -> tail
    | a :: tail -> a :: next tail
   
let fullyReact (input: string): string =
    let rec aux (units: char list) =
        let nextInput = next units
        if List.length units = List.length nextInput then
            nextInput
        else
            aux nextInput
    input
    |> Seq.toList
    |> aux
    |> List.map string
    |> String.concat ""
    
let p1 = fullyReact input |> String.length
printfn $"%A{p1}"