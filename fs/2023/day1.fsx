open System

let input = 
    System.IO.File.ReadLines($"{__SOURCE_DIRECTORY__}/day1.txt") 
    |> Seq.toList

let exampleInput =
    "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet".Split '\n' |> List.ofArray

let exampleInput2 =
    "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen".Split '\n' |> List.ofArray

let charToInt = Char.GetNumericValue >> int
let tupToInt (b, a) = a * 10 + b

let findDigit (str: string) =
    let folder acc nextChar  =
        match nextChar with
        | digit when Char.IsAsciiDigit digit -> charToInt digit :: acc
        | _ -> acc
    str.ToCharArray ()
    |> List.ofArray
    |> List.fold folder []
    |> (fun ints -> List.head ints, List.last ints)
    |> tupToInt


let solveP1 = List.map findDigit >> List.sum
let p1 = solveP1 input

// let numbers = Map [
//     ("one", 1)
//     ("two", 2)
//     ("three", 3)
//     ("four", 4)
//     ("five", 5)
//     ("six", 6)
//     ("seven", 7)
//     ("eight", 8)
//     ("nine", 9)
//     ("zero", 0)
// ]

// let isPartialMatch (str: string) =
//     numbers.Keys
//     |> Seq.exists (fun (key: string) -> key.StartsWith str)

// let exactMatch (str: string) = numbers.TryFind str

// let findDigit2 (str: string) =
//     let folder (partial, acc) nextChar =
//         match nextChar with
//         | digit when Char.IsAsciiDigit digit -> ([], charToInt digit :: acc)
//         | letter ->
//             let next = letter :: partial
//             let maybeNumber = next |> List.rev |> String.Concat
//             if isPartialMatch maybeNumber then
//                 match exactMatch maybeNumber with
//                 | Some value -> ([], value :: acc)
//                 | None -> (next, acc)
//             else
//                 ([letter], acc)

//     str.ToCharArray ()
//     |> List.ofArray
//     |> List.fold folder ([], [])
//     |> snd
//     |> (fun ints -> if ints.Length = 1 then 0, 0 else List.head ints, List.last ints)
//     |> tupToInt

// let solveP2 = List.map findDigit2 >> List.sum
// let p2 = solveP2 input
