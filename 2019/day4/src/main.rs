#![allow(unused)]

use aoc::Solution;
use std::env;
use std::ops::Range;

fn main() {
    let input = env::args()
        .nth(1)
        .expect("Missing day 4 input as argument.");

    let passwords = Part1::solve(&input);
    println!("Part 1: {}", passwords.len());
    println!("Part 2: {}", Part2::solve(passwords));
}

struct Part1;
struct Part2;

impl Solution<&str, Vec<Password>> for Part1 {
    fn solve(input: &str) -> Vec<Password> {
        let range = to_range(input).expect("Failed to parse range from args.");
        range.into_iter().filter_map(|x| {
            let result = is_valid_p1(&x);
            match result {
                (false, _) => None,
                (true, ch) => Some(Password { value: x, repeated: ch })
            }
        }).collect::<Vec<_>>()
    }
}

impl Solution<Vec<Password>, usize> for Part2 {
    fn solve(input: Vec<Password>) -> usize {
        input.into_iter().filter(adj_only_two).count()
    }
}

fn adj_only_two(password: &Password) -> bool {
    let chars = to_char_vec(password.value);
    
    let mut count = 0;
    for ch in chars {
        if ch != password.repeated {
            continue;
        }

        count += 1;
    }
    count == 2
}

fn to_char_vec(val: isize) -> Vec<char> {
    val.to_string()
        .chars()
        .collect()
}

fn is_valid_p1(val: &isize) -> (bool, char) {
    fn never_decreases(digits: &[char]) -> bool {
        let first = digits.iter();
        let second = digits.iter().skip(1);

        first.zip(second).all(|(d1, d2)| d1 <= d2)
    }

    fn has_adjacent(digits: &[char]) -> (bool, char) {
        let first = digits.iter();
        let second = digits.iter().skip(1);

        let mut repeated = ' ';
        let found = first.zip(second).any(|(d1, d2)| {
            if d1 == d2 {
                repeated = *d1;
            }

            d1 == d2
        });

        (found, repeated)
    }

    let digits = to_char_vec(*val);

    let (adjacent, repeated) = has_adjacent(&digits);
    (*val < 10_000_000 && never_decreases(&digits) && adjacent, repeated)
}

fn to_range(input: &str) -> Option<Range<isize>> {
    let parts: Vec<_> = input.split("-").collect();

    match (parts[0].parse(), parts[1].parse()) {
        (Ok(start), Ok(end)) => Some(start..end),
        _ => None,
    }
}

struct Password {
    value: isize,
    repeated: char,
}