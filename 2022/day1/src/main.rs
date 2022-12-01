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
    let mut sums: Vec<usize> = input
        .as_ref()
        .split("\n\n")
        .map(|ls| ls.lines())
        .map(|ls| ls.filter_map(|num| num.parse::<usize>().ok()))
        .map(|cs| cs.sum())
        .collect();

    sums.sort_by(|a, b| b.cmp(a));
    let p1 = sums[0];
    let p2 = sums[0..3].iter().sum();

    (p1, p2)
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn provided_p1() {
        assert_eq!((24000, 45000), solve(PROVIDED));
    }
}
