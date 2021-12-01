use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<usize> = read_to_string("input.txt")?
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    println!("Day 1: {} scans were greater than their previous entry!", solve(&input, 1));
    println!("Day 2: {} scan groups were greater than the previous group!", solve(&input, 3));

    Ok(())
}

pub fn solve(input: &[usize], window_size: usize) -> usize {
    fn sum(slice: &[usize]) -> usize {
        slice
            .iter()
            .sum()
    }

    input
        .windows(window_size)
        .zip(input[1..].windows(window_size))
        .inspect(|(_first, _second)| {
            #[cfg(test)] //only print debug output in test configuration
            eprintln!("first: Σ{:?} = {}\nsecond: Σ{:?} = {}", _first, sum(_first), _second, sum(_second));
        })
        .filter(|(first, second)| sum(second) > sum(first))
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    const PROVIDED_INPUT: [usize; 10] = [199,
        200,
        208,
        210,
        200,
        207,
        240,
        269,
        260,
        263,
    ];

    #[test]
    fn provided_p1() {
        assert_eq!(7, solve(&PROVIDED_INPUT, 1))
    }

    #[test]
    fn provided_p2() {
        assert_eq!(5, solve(&PROVIDED_INPUT, 3))
    }
}