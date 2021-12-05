use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>>{
    let input: Vec<String> = read_to_string("input.txt")?
        .lines()
        .map(ToOwned::to_owned)
        .collect();

    println!("Part 1: {}", solve(&input));
    Ok(())
}

fn solve(input: &[impl AsRef<str>]) -> i32 {
    let mask = "1".repeat(input[0].as_ref().len());
    let mask = i32::from_str_radix(&mask, 2).unwrap();

    let gamma = (0..input[0].as_ref().len()).map(|pos| {
        input.iter()
            .map(AsRef::as_ref)
            .filter(|s| s.chars().nth(pos) == Some('0'))
            .count()
    }).fold(String::new(), |acc, count| {
        if count > input.len() / 2 {
            acc + "0"
        } else {
            acc + "1"
        }
    });

    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = gamma ^ mask;
    gamma * epsilon
}

#[cfg(test)]
mod test {
    use super::*;

    const PROVIDED: [&str; 12] = [
        "00100",
        "11110",
        "10110",
        "10111",
        "10101",
        "01111",
        "00111",
        "11100",
        "10000",
        "11001",
        "00010",
        "01010",
    ];

    #[test]
    fn provided_p1() {
        assert_eq!(198, solve(&PROVIDED))
    }
}