use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>>{
    let input: Vec<String> = read_to_string("input.txt")?
        .lines()
        .map(ToOwned::to_owned)
        .collect();

    println!("Day 1: Horizontal * Depth = {}", solve(&input));
    Ok(())
}

fn solve(input: &[impl AsRef<str>]) -> isize {
    let mut depth = 0;
    let mut pos = 0;

    for directive in input {
        let mut parts = directive.as_ref().split(" ");

        let command = parts.nth(0).unwrap();
        let amount: isize = parts.nth(0).and_then(|amt| amt.parse().ok()).unwrap();

        match command {
            "forward" => pos += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => continue,
        }
    }

    depth * pos
}

#[cfg(test)]
mod test {
    use super::*;

    const PROVIDED: [&str; 6] = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    #[test]
    fn provided_p1() {
        assert_eq!(150, solve(&PROVIDED))
    }
}