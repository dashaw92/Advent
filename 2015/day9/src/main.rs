use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("Part 1: {}", solve_p1(&input));
    Ok(())
}

fn solve_p1(input: impl AsRef<str>) -> i32 {
    0
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn provided_p1() {
        assert_eq!(605, solve_p1(PROVIDED));
    }
}
