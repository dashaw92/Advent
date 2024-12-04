open System
open System.Text.RegularExpressions

let despace (line: string) = Regex.Replace (line, " {2,}", " ")

let rf path = IO.File.ReadLines($"{__SOURCE_DIRECTORY__}/inputs/{path}") |> String.concat "\n"

let lines (str: string) = str.Split('\n')

let toCharGrid = lines >> Array.map _.ToCharArray()
