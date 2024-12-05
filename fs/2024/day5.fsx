#load "AoCShared.fsx"

open AoCShared
open System

type Rules = Map<int, int list>

let splitRule (rule: string) =
    let sp = rule.Split '|'
    int sp[0], int sp[1]

let toRules (ruleStr: string): Rules =
    let toRuleMap (m: Rules) (key, value) =
        Map.change key (function
            | Some ls -> Some (value :: ls) 
            | None -> Some [value]
        ) m
         
    ruleStr.Split '\n'
    |> Array.map splitRule
    |> Array.fold toRuleMap Map.empty

let toNums (numLines: string) =
    numLines.Split '\n'
    |> Array.map (fun line -> line.Split ',')
    |> Array.map (Array.map int)

let toDay5 (str: string) =
    let split = str.Split "\n\n"
    toRules split[0], toNums split[1]

// let (rules, reports) = rf "day5.txt" |> toDay5

let (rulesEx, reportsEx) = toDay5 "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"

printfn $"%A{rulesEx}"
printfn $"%A{reportsEx}"
