use aoc::Plumb;

use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (p1, p2) = solve(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

fn solve(input: impl AsRef<str>) -> (usize, usize) {
    let mut adapters: Vec<i32> = input.plumb();
    adapters.sort();

    let max = *adapters.last().unwrap();

    let mut ones = 0;
    let mut threes = 1; //account for the builtin adapter
    let mut current = 0;
    
    while current != max {
        let next = adapters.iter()
            .filter(|&&jolts| [1,2,3].contains(&(jolts - current)))
            .min().unwrap();
        match next.abs_diff(current) {
            1 => ones += 1,
            3 => threes += 1,
            _ => {},
        }

        current = *next;
    }

    (ones * threes, 0)
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "16
10
15
5
1
11
7
19
6
12
4";

    #[test]
    fn provided_p1() {
        assert_eq!((35, 0), solve(PROVIDED));
    }
}
