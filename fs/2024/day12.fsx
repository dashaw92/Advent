#load "AoCShared.fsx"

open AoCShared
open System

let input = rf "day12.txt" |> toCharGrid
let ex = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE" |> toCharGrid

let neighbors grid (x, y) =
   let wh = dims grid 

   [-1, 0; 1, 0; 0, -1; 0, 1]
   |> List.map (fun (a, b) -> (x + a, y + b))
   |> List.filter (inGrid wh)

let search grid pos =
   let it = atGrid grid pos
   let rec aux found = function
      | next :: nq ->
         let maybeNext =
            neighbors grid next
            |> List.filter (atGrid grid >> ((=)it))
            |> List.filter (fun p -> (not <| List.contains p found) && (not <| List.contains p nq))

         let nf = next :: found
         aux (nf |> List.filter (atGrid grid >> ((=)it))) (List.append nq maybeNext)       
      | [] -> found

   aux [pos] (neighbors grid pos)

let fenceScore grid pos =
   let it = atGrid grid pos
   let similar =
      neighbors grid pos
      |> List.filter (atGrid grid >> (=)it)
      |> List.length
   4 - similar

let getScore grid plot =
   let size = List.length plot
   let perim = 
      plot
      |> List.map (fenceScore grid)
      |> List.sum
   size * perim

let allPlots grid =
   iterGrid grid
   |> List.map fst
   |> List.map (search grid)
   |> List.map List.sort
   |> List.distinct

let solveP1 grid =
   allPlots grid
   |> List.map (getScore grid)
   |> List.sum

// allPlots ex
// |> List.map (fun plot -> (List.length plot, plot), (atGrid ex (List.head plot)))
// |> List.filter (snd >> (=)'R')
// |> printfn "%A"
// printfn "%A" <| getScore ex r
printfn "%A" <| solveP1 input
