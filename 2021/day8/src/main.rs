use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("Part 1: {}", solve_p1(&input));
    Ok(())
}

fn solve_p1(input: impl AsRef<str>) -> usize {
    0
}

fn solve_p2(input: impl  AsRef<str>) -> usize {
    0
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "";

    #[test]
    fn provided_p1() {
        assert_eq!(0, solve_p1(PROVIDED));
    }

    #[test]
    fn provided_p2() {
        assert_eq!(0, solve_p2(PROVIDED));
    }
}
