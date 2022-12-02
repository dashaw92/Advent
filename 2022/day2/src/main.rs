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

    let (p1, p2) = solve(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

fn solve(input: impl AsRef<str>) -> (usize, usize) {
    let input: Vec<Round> = input.plumb();

    let p1: usize = input.iter().map(Round::score_p1).sum();
    let p2: usize = input.iter().map(Round::score_p2).sum();
    (p1, p2)
}

struct Round {
    them: Choice,
    me: Choice,
    p2_outcome: Outcome,
}

impl Round {
    fn score_p1(&self) -> usize {
        let choice_val = self.me.value();
        let outcome = self.me.outcome(&self.them).value();

        choice_val + outcome
    }

    fn score_p2(&self) -> usize {
        let outcome = self.p2_outcome.value();

        let choice_val = match self.p2_outcome {
            Outcome::Win => self.them.foiled_by().value(),
            Outcome::Draw => self.them.value(),
            Outcome::Loss => self.them.wins_against().value(),
        };

        choice_val + outcome
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (them, me) = s.split_once(' ').expect("invalid input");

        let p2_outcome: Outcome = me.parse().unwrap();
        let (them, me): (Choice, Choice) = (them.parse().unwrap(), me.parse().unwrap());

        Ok(Round {
            them,
            me,
            p2_outcome,
        })
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

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().chars().next() {
            Some(c) => Ok(match c {
                'X' => Outcome::Loss,
                'Y' => Outcome::Draw,
                'Z' => Outcome::Win,
                _ => return Err(()),
            }),
            _ => Err(()),
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

    fn foiled_by(&self) -> Choice {
        use Choice::*;

        match self {
            Rock => Paper,
            Scissors => Rock,
            Paper => Scissors,
        }
    }

    fn wins_against(&self) -> Choice {
        use Choice::*;

        match self {
            Rock => Scissors,
            Scissors => Paper,
            Paper => Rock,
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
        assert_eq!((15, 12), solve(PROVIDED));
    }
}
