open System

type Mapper = int64 -> int64

let anyMapper (m1: Mapper) (m2: Mapper): Mapper =
    (fun i ->
        match m1 i with
        | out when out = i -> m2 i
        | mapped -> mapped
    )

let makeMapper (mappings: string): Mapper =
    (mappings.Split "\n")[1..] 
    |> Array.map (fun str -> str.Split " ") 
    |> Array.map (Array.map int64)
    |> Array.map (
        fun vals ->
            let dest = vals[0]
            let src = vals[1]
            let len = vals[2] - 1L

            (fun mapInput ->
                // printfn $"{dest} <- {src} - {src + len} ({len})"
                match mapInput with
                | outRange when outRange < src || outRange > (src + len) -> outRange
                | x -> dest + (x - src)
            )
        )
    |> Array.reduce anyMapper

let parseInput (input: string) =
    let sections = input.Split "\n\n"
    let seeds = (sections[0].Split " ")[1..] |> Array.map int64

    let chain = 
        sections[1..]
        |> Array.map makeMapper
        |> Array.reduce (>>)

    seeds, chain

let solveP1 =
    parseInput
    >> (fun (seeds, chain) -> seeds |> Array.map chain)
    >> Array.min

let rf path = IO.File.ReadLines($"{__SOURCE_DIRECTORY__}/{path}") |> String.concat "\n"

let input = rf "day5.txt"
let example = rf "day5ex.txt"