use std::{ops::RangeInclusive};

use aoc::*;

fn main() {
    let file = read_input("input.txt");
    let input: Vec<_> = file.lines().map(Policy::from).collect();

    println!("Part 1: {}", Part1::solve(&input));
    println!("Part 2: {}", Part2::solve(&input));
}

struct Part1;
struct Part2;

impl Solution<&[Policy], usize> for Part1 {

    fn solve(input: &[Policy]) -> usize {
        input.into_iter()
            .filter(|policy| policy.valid_password_p1())
            .count()
    }

}

impl Solution<&[Policy], usize> for Part2 {

    fn solve(input: &[Policy]) -> usize {
        input.into_iter()
            .filter(|policy| policy.valid_password_p2())
            .count()
    }
}

//A parsed representation of a line from the input.txt
//A full line looks like:
//1-3 a: axxvaaaz
//range: The first part of the input, the 1-3
//requirement: The char before the :
//password: The remainder of the line after the :, trimmed
struct Policy {
    range: RangeInclusive<usize>,
    requirement: char,
    password: String,
}

impl From<&str> for Policy {

    fn from(input: &str) -> Self {
        let mut split = input.split(':');

        //policy
        let first: Vec<_> = split.next().unwrap().split_whitespace().collect();
        let range = {
            let ints: Vec<_> = first[0].split('-')
                .filter_map(|int| int.parse::<usize>().ok())
                .collect();   
            ints[0] ..= ints[1]
        };

        let requirement = first[1].trim().chars().nth(0).unwrap();

        //password
        let password = String::from(split.next().unwrap());

        Self {
            range,
            requirement,
            password,
        }
    }

}

impl Policy {

    //just filter out all non-required chars and then count the resulting iterator
    fn valid_password_p1(&self) -> bool {
        let count = self.password.chars()
            .filter(|&ch| ch == self.requirement)
            .count();

        self.range.contains(&count)
    }

    //use the range's start and end to index the chars of the password.
    //Exactly one of the predicates must be true, so we XOR them together
    fn valid_password_p2(&self) -> bool {
        let chars: Vec<_> = self.password.chars().collect();
        let start = *self.range.start();
        let end = *self.range.end();

        (chars[start] == self.requirement) ^ (chars[end] == self.requirement)
    }

}