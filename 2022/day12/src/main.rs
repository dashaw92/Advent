mod map;
use map::*;
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
    let map = Map::new(input);

    let path = find_path(&map);
    print_path(&map, &path);
    let p1 = path.len();

    (p1, 0)
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn provided_p1() {
        assert_eq!((0, 0), solve(PROVIDED));
    }
}
