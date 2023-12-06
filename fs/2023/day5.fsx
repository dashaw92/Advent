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
                match mapInput with
                | outRange when outRange < src || outRange > (src + len) -> outRange
                | x -> dest + (x - src)
            )
        )
    |> Array.reduce anyMapper

let parseSeedsP1 = id
let parseSeedsP2 =
    Seq.chunkBySize 2
    >> Seq.collect (fun range -> seq { range[0] .. (range[0] + range[1] - 1L) })

let parseInput seedParser (input: string) =
    let sections = input.Split "\n\n"
    let seeds = seedParser ((sections[0].Split " ")[1..] |> Seq.map int64)

    let chain = 
        sections[1..]
        |> Array.map makeMapper
        |> Array.reduce (>>)

    chain, seeds

let uncurry f (a, b) = f a b

let solve seedParser =
    parseInput seedParser
    >> uncurry Seq.map
    >> Seq.min

let solveP1 = solve parseSeedsP1
let solveP2 = solve parseSeedsP2

let rf path = IO.File.ReadLines($"{__SOURCE_DIRECTORY__}/{path}") |> String.concat "\n"

let input = rf "day5.txt"