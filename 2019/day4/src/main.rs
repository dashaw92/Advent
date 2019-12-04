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

impl Solution<&str, Vec<isize>> for Part1 {
    fn solve(input: &str) -> Vec<isize> {
        let range = to_range(input).expect("Failed to parse range from args.");
        range.into_iter().filter(is_valid_p1).collect::<Vec<_>>()
    }
}

impl Solution<Vec<isize>, usize> for Part2 {
    fn solve(input: Vec<isize>) -> usize {
        input.into_iter().filter(adj_only_two).count()
    }
}

fn adj_only_two(password: &isize) -> bool {
    use std::collections::HashMap;

    let chars = to_char_vec(*password);

    let mut map = HashMap::new();
    for ch in chars {
        *map.entry(ch).or_insert(0) += 1;
    }

    map.values().any(|&occurrences| occurrences == 2)
}

fn to_char_vec(val: isize) -> Vec<char> {
    val.to_string().chars().collect()
}

fn is_valid_p1(val: &isize) -> bool {
    fn never_decreases(digits: &[char]) -> bool {
        let first = digits.iter();
        let second = digits.iter().skip(1);

        first.zip(second).all(|(d1, d2)| d1 <= d2)
    }

    fn has_adjacent(digits: &[char]) -> bool {
        let first = digits.iter();
        let second = digits.iter().skip(1);

        first.zip(second).any(|(d1, d2)| d1 == d2)
    }

    let digits = to_char_vec(*val);

    *val < 10_000_000 && never_decreases(&digits) && has_adjacent(&digits)
}

fn to_range(input: &str) -> Option<Range<isize>> {
    let parts: Vec<_> = input.split("-").collect();

    match (parts[0].parse(), parts[1].parse()) {
        (Ok(start), Ok(end)) => Some(start..end),
        _ => None,
    }
}
