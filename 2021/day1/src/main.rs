use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<usize> = read_to_string("input.txt")?
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    println!("Day 1: {} scans were greater than their previous entry!", solve(&input));

    Ok(())
}

pub fn solve(input: &[usize]) -> usize {
    input.iter()
        .zip(input.iter().skip(1))
        .filter(|(&first, &second)| second > first)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn provided() {
        let input = [199,
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

        assert_eq!(7, solve(&input))
    }
}