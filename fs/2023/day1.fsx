open System

let input = 
    System.IO.File.ReadLines($"{__SOURCE_DIRECTORY__}/day1.txt") 
    |> Seq.toList

let exampleInput =
    "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet".Split '\n' |> List.ofArray

let findDigit (str: string) =
    let charToInt = Char.GetNumericValue >> int
    let tupToInt (a, b) = a * 10 + b
    let ints = 
        str.ToCharArray ()
        |> Array.filter Char.IsAsciiDigit
        |> Array.map charToInt
    tupToInt (Array.head ints, Array.last ints)

let solveP1 = List.map findDigit >> List.sum
let p1 = solveP1 input