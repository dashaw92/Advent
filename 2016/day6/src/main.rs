use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (p1, p2) = solve(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

type FreqMap = Vec<HashMap<char, u32>>;

fn build_freqs(input: impl AsRef<str>) -> FreqMap {
    let input: Vec<&str> = input.as_ref().lines().collect();
    let mut freq: FreqMap = Vec::with_capacity(input.len());
    for _ in 0..input[0].len() {
        freq.push(HashMap::new());
    }

    for l in input {
        for (i, c) in l.chars().enumerate() {
            *freq[i].entry(c).or_insert(0) += 1;
        }
    }

    freq
}

fn solve(input: impl AsRef<str>) -> (String, String) {
    let freq = build_freqs(input);

    let p1 = freq
        .iter()
        .map(|map| map.iter().max_by_key(|(_, count)| **count).unwrap())
        .map(|(c, _)| *c)
        .collect();
    let p2 = freq
        .iter()
        .map(|map| map.iter().min_by_key(|(_, count)| **count).unwrap())
        .map(|(c, _)| *c)
        .collect();

    (p1, p2)
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
        assert_eq!(("easter".to_owned(), "advent".to_owned()), solve(PROVIDED));
    }
}
