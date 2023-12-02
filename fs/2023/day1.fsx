open System

let input = 
    System.IO.File.ReadLines($"{__SOURCE_DIRECTORY__}/day1.txt") 
    |> Seq.toList

let findDigit (str: string) =
    let charToInt = Char.GetNumericValue >> int
    
    str.ToCharArray ()
    |> Array.filter Char.IsAsciiDigit
    |> Array.map charToInt
    |> (fun ints -> Array.head ints * 10 + Array.last ints)

let filterP2 (input: string) =
    let replace (a: string) (b: string) (str: string) = str.Replace (a, b)
    replace "one" "o1e" input
    |> replace "two" "t2o"
    |> replace "three" "t3e"
    |> replace "four" "f4r"
    |> replace "five" "f5e"
    |> replace "six" "s6x"
    |> replace "seven" "s7n"
    |> replace "eight" "e8t"
    |> replace "nine" "n9e"

let solveP1 = List.map findDigit >> List.sum
let solveP2 = List.map (filterP2 >> findDigit) >> List.sum
let p1 = solveP1 input
let p2 = solveP2 input