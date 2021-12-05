use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>>{
    let input: Vec<String> = read_to_string("input.txt")?
        .lines()
        .map(ToOwned::to_owned)
        .collect();

    let (p1, p2) = solve(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

fn solve(input: &[impl AsRef<str>]) -> (i32, i32) {
    let input: Vec<&str> = input.iter().map(AsRef::as_ref).collect();
    let len = input[0].len();

    let mask = "1".repeat(len);
    let mask = i32::from_str_radix(&mask, 2).unwrap();

    let gamma = (0..len).map(|pos| {
        input.iter()
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

    let oxygen = i32::from_str_radix(drain(&input, true), 2).unwrap();
    let co2 = i32::from_str_radix(drain(&input, false), 2).unwrap();

    (gamma * epsilon, oxygen * co2)
}

fn drain<'a>(input: &'a [&'a str], common: bool) -> &'a str {
    let mut oxygen: Vec<&str> = input.to_vec();

    (0..input.len()).for_each(|pos| {
        if oxygen.len() == 1 {
            return
        }

        let pos_count = oxygen.iter()
            .filter(|s| s.chars().nth(pos) == Some('1'))
            .count();

        let ret = if oxygen.len() - pos_count <= pos_count {
            if common {
                Some('1')
            } else {
                Some('0')
            }
        } else {
            if common {
                Some('0')
            } else {
                Some('1')
            }
        };

        oxygen.retain(|s| s.chars().nth(pos) == ret);
    });

    oxygen[0]
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
        assert_eq!((198, 230), solve(&PROVIDED))
    }
}