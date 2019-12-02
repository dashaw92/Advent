use aoc::*;

fn main() {
    let file = read_input("input.txt");
    let mut input: Vec<u32> = file
        .split_terminator(',')
        .flat_map(|digit| digit.parse().ok())
        .collect();

    println!("Part 1: {}", Part1::solve(&mut input));
    println!("Part 2: {}", Part2::solve(&mut input));
}

struct Part1;
struct Part2;

impl Solution<&mut Vec<u32>, u32> for Part1 {
    fn solve(input: &mut Vec<u32>) -> u32 {
        input[1] = 12;
        input[2] = 2;

        run_tape(input.clone())[0]
    }
}

impl Solution<&mut Vec<u32>, u32> for Part2 {
    fn solve(input: &mut Vec<u32>) -> u32 {
        let mut final_noun = 0;
        let mut final_verb = 0;

        'outer: for noun in 0..=99 {
            for verb in 0..=99 {
                let mut copy = input.clone();
                copy[1] = noun;
                copy[2] = verb;

                let output = run_tape(copy);
                if output[0] == 19690720 {
                    final_noun = noun;
                    final_verb = verb;
                    break 'outer;
                }
            }
        }
        
        100 * final_noun + final_verb
    }
}

fn is_opcode(val: u32) -> bool {
    [1,2,99].contains(&val)
}

fn run_tape(mut input: Vec<u32>) -> Vec<u32> {
    let mut pc = 0;
    loop {
        if !is_opcode(input[pc]) {
            pc += 1;
            continue;
        }

        let oper = input[pc] as usize;
        let val1 = input[pc + 1] as usize;
        let val2 = input[pc + 2] as usize;
        let dest = input[pc + 3] as usize;

        match oper {
            1 => input[dest] = input[val1] + input[val2],
            2 => input[dest] = input[val1] * input[val2],
            99 => break,
            _ => {},
        }
        
        pc += 4;
    }

    input.clone()
}