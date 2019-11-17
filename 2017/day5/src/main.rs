use std::fs::read_to_string;
use std::io::Result;

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let tape = input
        .lines()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<_>>();

    println!("The answer to part 1 is {}", solve(tape.clone(), false));
    println!("The answer to part 2 is {}", solve(tape.clone(), true));
    Ok(())
}

fn solve(mut tape: Vec<i32>, part2: bool) -> i32 {
    let mut pc: i32 = 0;
    let mut steps = 1;
    loop {
        let cell = tape[pc as usize];
        if part2 && cell >= 3 {
            tape[pc as usize] -= 1;
        } else {
            tape[pc as usize] += 1;
        }
        pc += cell; //Jump to the position
        if pc < 0 {
            pc = tape.len() as i32 - pc - 1;
        }
        if pc > tape.len() as i32 - 1 {
            break;
        }
        steps += 1;
    }

    steps
}
