#load "AoCShared.fsx"
open AoCShared
open System

type Card = CJoker | C2 | C3 | C4 | C5 | C6 | C7 | C8 | C9 | CT | CJ | CQ | CK | CA
type Hand = { Bid: int; Cards: Card list }
type HandType = Five | Four | House | Three | Two | One | High

let cardOf isP2 = function
    | '2' -> C2
    | '3' -> C3
    | '4' -> C4
    | '5' -> C5
    | '6' -> C6
    | '7' -> C7
    | '8' -> C8
    | '9' -> C9
    | 'T' -> CT
    | 'J' when isP2 -> CJoker
    | 'J' -> CJ
    | 'Q' -> CQ
    | 'K' -> CK
    | 'A' -> CA
    | bad -> failwithf $"Bad card type: {bad}"
let parseHand p2 (str: string) = str.ToCharArray() |> Array.map (cardOf p2) |> List.ofArray
let parseInput p2 (str: string) = 
    let sp = str.Split " "
    let hand = parseHand p2 sp[0]
    let bid = sp[1] |> int

    { Bid = bid; Cards = hand }

let countCards hand =
    hand.Cards
    |> List.map (fun card -> card, (List.filter ((=) card) hand.Cards |> List.length))
    |> List.distinctBy (fun (card, _) -> card)
    |> Map

let classify mapper hand =
    let hand = mapper hand
    let count = countCards hand
    let hasSetOf num = Map.values count |> Seq.exists ((=) num)
    let distinctCards = Map.keys count |> Seq.length
    
    if hasSetOf 5 then Five
    else if hasSetOf 4 then Four
    else if hasSetOf 3 then
        if distinctCards = 2 then House
        else Three
    else if hasSetOf 2 then
        if distinctCards = 3 then Two
        else One
    else High

let expandJokers hand =
    let count = countCards hand
    let numJokers = defaultArg (Map.tryFind CJoker count) 0
    match numJokers with
    | 0 -> hand
    | 5 -> { hand with Cards = [C2; C2; C2; C2; C2]}
    | x ->
        let commonCard = Map.toSeq count |> Seq.filter (fst >> (<>) CJoker) |> Seq.maxBy snd |> fst
        let replacedCards = 
            hand.Cards 
            |> List.map (fun card -> if card = CJoker then commonCard else card)
        { hand with Cards = replacedCards }

let cmpHands classifier h1 h2 =
    if h1 = h2 then 0 else
    let tH1 = classifier h1
    let tH2 = classifier h2

    if tH1 <> tH2 then compare tH1 tH2
    else
        List.zip h1.Cards h2.Cards
        |> List.find (fun (c1, c2) -> c1 <> c2)
        |> (fun (c1, c2) -> compare c2 c1)

let solve parser mapper =
    List.map parser
    >> List.sortWith (cmpHands (classify mapper))
    >> List.rev
    >> List.mapi (fun ranking hand -> (ranking + 1) * hand.Bid)
    >> List.sum

let solveP1 = solve (parseInput false) id
let solveP2 = solve (parseInput true) expandJokers

let input = (rf "day7.txt").Split '\n' |> List.ofArray

let p1 = solveP1 input
let p2 = solveP2 input