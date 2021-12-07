use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("Part 1: Hit {} trees!", solve(&input));
    Ok(())
}

fn solve(input: impl AsRef<str>) -> usize {
    let input: Vec<&str> = input.as_ref().lines().collect();

    let mut pos: (usize, usize) = (0, 0);
    let mut count = 0;
    loop {
        let idx = input[pos.1].chars().nth(pos.0).unwrap();
        if idx == '#' {
            count += 1;
        }

        if pos.1 + 1 >= input.len() {
            break;
        }

        pos.1 += 1;
        pos.0 = (pos.0 + 3) % input[pos.1].len();
    }

    count
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