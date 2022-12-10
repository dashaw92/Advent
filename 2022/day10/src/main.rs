use aoc::Plumb;

use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (p1, p2) = solve(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

fn solve(input: impl AsRef<str>) -> (i32, i32) {
    let input: Vec<Instr> = input.plumb();
    let breakpoints = [20, 60, 100, 140, 180, 220];

    let mut idx = 0;
    let mut total = 0;
    let mut cycles = 0;
    let mut x = 1;

    let mut strengths = vec![];

    while idx < input.len() {
        total += 1;

        if breakpoints.contains(&total) {
            strengths.push(total * x);
        }

        let instr = &input[idx];
        if cycles + 1 == instr.cycles() {
            match instr {
                Instr::Noop => {}
                Instr::Add(val) => x += val,
            }

            idx += 1;
            cycles = 0;
            continue;
        }

        cycles += 1;
    }

    let p1 = strengths.iter().sum();
    (p1, 0)
}

enum Instr {
    Add(i32),
    Noop,
}

impl Instr {
    fn cycles(&self) -> usize {
        match self {
            Instr::Add(_) => 2,
            Instr::Noop => 1,
        }
    }
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Instr::Noop);
        }

        let add = s.split_once(' ').and_then(|(_, x)| x.parse().ok()).unwrap();
        Ok(Instr::Add(add))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = include_str!("../provided.txt");

    #[test]
    fn provided_p1() {
        assert_eq!((13140, 0), solve(PROVIDED));
    }
}
