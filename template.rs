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
    let input: Vec<i32> = input.plumb();
    (0, 0)
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "";

    #[test]
    fn provided_p1() {
        assert_eq!((0, 0), solve(PROVIDED));
    }
}
