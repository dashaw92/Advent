/**
* 1st
* A Rock
* B Paper
* C Scissors
*
* 2nd
* X Rock
* Y Paper
* Z Scissors
*
* Winner : Highest score
* Total score is sum of scores from every round
* Score for a single round is:
*     score for the shape selected (iota 1 Rock, Paper, Scissors)
*     + outcome (0 for loss, 3 for draw, 6 for win)
**/
use aoc::Plumb;

use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (p1, _p2) = solve(&input);
    println!("Part 1: {}", p1);
    Ok(())
}

fn solve(input: impl AsRef<str>) -> (usize, usize) {
    let input: Vec<Round> = input.plumb();

    let p1: usize = input.iter().map(Round::score).sum();
    (p1, 0)
}

struct Round {
    them: Choice,
    me: Choice,
}

impl Round {
    fn score(&self) -> usize {
        let choice_val = self.me.value();
        let outcome = self.me.outcome(&self.them).value();

        choice_val + outcome
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (them, me) = s.split_once(' ').expect("invalid input");
        let (them, me) = (them.parse().unwrap(), me.parse().unwrap());

        Ok(Round { them, me })
    }
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn value(&self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

#[derive(PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn value(&self) -> usize {
        use Choice::*;

        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn outcome(&self, other: &Choice) -> Outcome {
        use Choice::*;

        match (self, other) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Outcome::Win,
            (x, y) if x == y => Outcome::Draw,
            _ => Outcome::Loss,
        }
    }
}

impl FromStr for Choice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().chars().next() {
            Some(c) => Ok(match c {
                'A' | 'X' => Choice::Rock,
                'B' | 'Y' => Choice::Paper,
                'C' | 'Z' => Choice::Scissors,
                _ => return Err(()),
            }),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "A Y
B X
C Z";

    #[test]
    fn provided_p1() {
        assert_eq!((15, 0), solve(PROVIDED));
    }
}
