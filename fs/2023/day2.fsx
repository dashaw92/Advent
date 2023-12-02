open System

type Dice =
    | Red
    | Green
    | Blue

type Pick = int * Dice

type Game =  { Id: int; Rounds: Pick list list }

type Restriction = Pick list

let parsePick (pick: string): Pick =
    let sp = pick.Split ' '
    let roll = sp[0] |> int
    let color = 
        match sp[1] with
        | "red" -> Red
        | "green" -> Green
        | "blue" -> Blue
        | _ -> failwithf $"Bad color: {sp}"
    roll, color

let parseGame (line: string) =
    let sp = line.Split ": "
    let id = sp[0].Substring "Game ".Length |> int
    
    let rounds = 
        sp[1].Split "; "
        |> Array.map (fun fullGame -> fullGame.Split ", ")
        |> Array.map (Array.map parsePick)
        |> Array.map List.ofArray
        |> List.ofArray
    
    { Id = id; Rounds = rounds }

let getRestriction die restriction =
    List.tryFind (fun (_, color) -> color = die) restriction
    |> Option.map fst
    |> Option.defaultValue 1000

let isValid restriction game =
    let isValidRound restriction round =
        round |> List.forall (fun (roll, color) -> getRestriction color restriction >= roll)

    game.Rounds |> List.forall (isValidRound restriction)

let fewestCubes game =
    let updateMap (state: Map<Dice, int>) (roll, color) =
        let currentMax = defaultArg (state.TryFind color) 0
        if currentMax < roll then
            Map.add color roll state
        else state

    let out = 
        game.Rounds
        |> List.map (List.fold updateMap Map.empty)

    let max color =
        out |> List.maxBy (fun roundMap -> defaultArg (roundMap.TryFind color) 0)

    let getColor color (mapping: Map<Dice, int>) = defaultArg (mapping.TryFind color) 0

    let red = max Red |> getColor Red
    let green = max Green |> getColor Green
    let blue = max Blue |> getColor Blue
    
    [ (red, Red); (green, Green); (blue, Blue) ]

let solveP1 = 
    let p1Restriction = [ (12, Red); (13, Green); (14, Blue) ]

    Array.map parseGame
    >> Array.filter (isValid p1Restriction)
    >> Array.map (_.Id)
    >> Array.sum

let solveP2 =
    Array.map parseGame
    >> Array.map (fewestCubes >> List.map fst >> List.reduce ( * ))
    >> Array.sum

let input = 
    System.IO.File.ReadLines($"{__SOURCE_DIRECTORY__}/day2.txt") 
    |> Seq.toArray

let p1 = solveP1 input
let p2 = solveP2 input