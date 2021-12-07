use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("Part 1: Hit {} trees!", solve(&input, (3, 1)));
    println!("Part 2: Hit {} trees!", solve_p2(&input));
    Ok(())
}

fn solve(input: impl AsRef<str>, slope: (usize, usize)) -> usize {
    let input: Vec<&str> = input.as_ref().lines().collect();

    let mut pos: (usize, usize) = (0, 0);
    let mut count = 0;
    loop {
        let idx = input[pos.1].chars().nth(pos.0).unwrap();
        if idx == '#' {
            count += 1;
        }

        pos.1 += slope.1;
        if pos.1 >= input.len() {
            break;
        }

        pos.0 = (pos.0 + slope.0) % input[pos.1].len();
    }

    count
}

fn solve_p2(input: impl AsRef<str>) -> usize {
    [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ].into_iter().map(|slope| solve(&input, slope)).product()
}

#[cfg(test)]
mod test {
    use super::*;

    const PROVIDED: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn provided_p1() {
        assert_eq!(7, solve(&PROVIDED));
    }
}