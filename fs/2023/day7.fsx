#load "AoCShared.fsx"
open AoCShared
open System

type Card = C2 | C3 | C4 | C5 | C6 | C7 | C8 | C9 | CT | CJ | CQ | CK | CA
type Hand = { Bid: int; Cards: Card list }
type HandType = Five | Four | House | Three | Two | One | High

let cardOf = function
    | '2' -> C2
    | '3' -> C3
    | '4' -> C4
    | '5' -> C5
    | '6' -> C6
    | '7' -> C7
    | '8' -> C8
    | '9' -> C9
    | 'T' -> CT
    | 'J' -> CJ
    | 'Q' -> CQ
    | 'K' -> CK
    | 'A' -> CA
    | bad -> failwithf $"Bad card type: {bad}"
let parseHand (str: string): Card list = str.ToCharArray() |> Array.map cardOf |> List.ofArray
let parseInput (str: string) = 
    let sp = str.Split " "
    let hand = parseHand sp[0]
    let bid = sp[1] |> int

    { Bid = bid; Cards = hand }

let countCards hand =
    hand.Cards
    |> List.map (fun card -> card, (List.filter ((=) card) hand.Cards |> List.length))
    |> List.distinctBy (fun (card, _) -> card)
    |> Map

let classify hand =
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

let cmpHands h1 h2 =
    let tH1 = classify h1
    let tH2 = classify h2

    if tH1 <> tH2 then compare tH1 tH2
    else
        List.zip h1.Cards h2.Cards
        |> List.find (fun (c1, c2) -> c1 <> c2)
        |> (fun (c1, c2) -> compare c2 c1)

let solveP1 =
    List.map parseInput
    >> List.sortWith cmpHands
    >> List.rev
    >> List.mapi (fun ranking hand -> (ranking + 1) * hand.Bid)
    >> List.sum

let input = (rf "day7.txt").Split '\n' |> List.ofArray
let example =
    "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483".Split '\n' |> List.ofArray

// let test = parseHand >> (fun hand -> { Bid = 0; Cards = hand })
// let countIt = parseHand >> (fun hand -> { Bid = 0; Cards = hand }) >> countCards