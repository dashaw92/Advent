use aoc::Plumb;

use std::error::Error;
use std::fmt::Display;
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

    let mut crt = Crt::new();
    let mut idx = 0;
    let mut total = 0;
    let mut cycles = 0;
    let mut x = 1i32;

    let mut strengths = vec![];

    while idx < input.len() {
        let sprite = [x - 1, x, x + 1];
        crt.push(sprite.contains(&((crt.cycle % 40) as i32)));

        total += 1;
        crt.cycle();

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
    println!("{crt}");
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

struct Crt {
    cycle: usize,
    pix: [bool; 40 * 6],
}

impl Crt {
    fn new() -> Self {
        Crt {
            cycle: 0,
            pix: [false; 40 * 6],
        }
    }

    fn cycle(&mut self) {
        self.cycle += 1;
    }

    fn push(&mut self, on: bool) {
        self.pix[self.cycle] = on;
    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..6 {
            for x in 0..40 {
                write!(
                    f,
                    "{}",
                    match self.pix[y * 40 + x] {
                        true => "#",
                        false => ".",
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
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
