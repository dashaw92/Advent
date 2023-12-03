(*
Module for parsing (a subset) of JSON
Only needs to parse arrays, objects, strings, and numbers
*)

type JSON =
    | JSArray of JSON list
    | JSObject of JSON
    | JSString of string
    | JSInt of int

type ParseOut<'a> = ('a * string) option
type Parser<'a> = string -> ParseOut<'a>

let parseChar ch: Parser<char> =
    let parser (input: string) =
        if input.Length = 0 then None
        else
            match input[0] with
            | x when x = ch -> Some (x, input[1..])
            | _ -> None
    parser

module Operators =
    let andThen p1 p2: Parser<'a * 'a> =
        let parser (input: string) =
            match p1 input with
            | None -> None
            | Some (v1, remaining1) ->
                match p2 remaining1 with
                | None -> None
                | Some (v2, remaining2) -> Some ((v1, v2), remaining2)
        parser

    let orElse (p1: Parser<'a>) (p2: Parser<'a>): Parser<'a> =
        let parser (input: string) =
            match p1 input with
            | None -> p2 input
            | success -> success
        parser
        
    let map (p1: Parser<'a>) (mapper: 'a -> 'b): Parser<'b> =
        let parser (input: string) =
            match p1 input with
            | None -> None
            | Some (a, remaining) -> Some (mapper a, remaining)
        parser
    
    let (.>>.) = andThen
    let (<|>) = orElse
    let (|>>) = map
open Operators

let choice<'a> = List.reduce (<|>)
let anyOf charList = charList |> List.map parseChar |> choice

let sequence parsers =
    let cat p1 p2 =
        p1 .>>. p2 |>> (fun (l1, l2) -> l1 @ l2)
    parsers
    |> Seq.map (fun p -> p |>> List.singleton)
    |> Seq.reduce cat
    
let parseString str =
    str
    |> Seq.map parseChar
    |> sequence
    |>> (List.map string >> String.concat "")