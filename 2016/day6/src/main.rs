use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("Part 1: {}", solve_p1(&input));
    Ok(())
}

fn solve_p1(input: impl AsRef<str>) -> String {
    let input: Vec<&str> = input.as_ref().lines().collect();
    let mut freq: Vec<HashMap<char, i32>> = Vec::with_capacity(input.len());
    for _ in 0..input[0].len() {
        freq.push(HashMap::new());
    }

    for l in input {
        for (i, c) in l.chars().enumerate() {
            *freq[i].entry(c).or_insert(0) += 1;
        }
    }

    freq.iter()
        .map(|map| map.iter().max_by_key(|(_, count)| **count).unwrap())
        .map(|(c, _)| *c)
        .collect()
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    #[test]
    fn provided_p1() {
        assert_eq!("easter", &solve_p1(PROVIDED));
    }
}
