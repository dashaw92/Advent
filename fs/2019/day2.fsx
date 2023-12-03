module Intcode =
    type Ops =
        {
            Add: int -> int -> int
            Mul: int -> int -> int
            Halt: int
        }
    
    type State = { Ip: int; Tape: int list; Pc: int; Ops: Ops }
    
    let build tape ops = { Ip = 0; Tape = tape; Pc = 0; Ops = ops }

let input = 
    System.IO.File.ReadLines($"{__SOURCE_DIRECTORY__}/day2.txt")
    |> String.concat ""
    |> (fun st -> st.Split ",")
    |> (Array.toList >> List.map int)

let start = Intcode.build input

let rec solve (state: Intcode.State) =
    match state.Tape[state.Ip] with
    | 1 -> solve { state with Ip = state.Ip + 4; Pc = state.Pc + 1 }
    | 2 -> solve { state with Ip = state.Ip + 4; Pc = state.Pc + 1 }
    | 99 -> state
    | _ -> failwithf "Bad intcode at %A" {| Ip = state.Ip; Pc = state.Pc |}

let ops: Intcode.Ops =
    {
        Add = (+);
        Mul = (*);
        Halt = 99
    }

let p1 = solve (start ops)