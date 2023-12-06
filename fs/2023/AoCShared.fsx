open System
open System.Text.RegularExpressions

let despace (line: string) = Regex.Replace (line, " {2,}", " ")

let rf path = IO.File.ReadLines($"{__SOURCE_DIRECTORY__}/{path}") |> String.concat "\n"