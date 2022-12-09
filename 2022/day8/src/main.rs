use aoc::grid::Grid;

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
    let g = get_grid(input);
    let (w, h) = g.size();

    println!("{g}");
    let p1 = (0..w * h)
        .map(|idx| g.to_xy(idx))
        .filter(|&pos| is_vis(&g, pos))
        .count();

    (p1, 0)
}

fn is_vis(g: &Grid<u32>, (x, y): (usize, usize)) -> bool {
    let (w, h) = g.size();
    let elem = g[(x, y)];
    if x == 0 || y == 0 || x == w - 1 || y == h - 1 {
        return true;
    }

    let left = (0..x).map(|nx| g[(nx, y)]).all(|tree| tree < elem);
    let right = (x + 1..w).map(|nx| g[(nx, y)]).all(|tree| tree < elem);

    let up = (0..y).map(|ny| g[(x, ny)]).all(|tree| tree < elem);
    let down = (y + 1..h).map(|ny| g[(x, ny)]).all(|tree| tree < elem);

    left || right || up || down
}

fn get_grid(input: impl AsRef<str>) -> Grid<u32> {
    let input = input.as_ref();
    let width = input.lines().next().map(str::len).expect("What?");
    let height = input.lines().count();
    let iter = input.chars().filter_map(|c| c.to_digit(10));
    Grid::from_iter(iter, width, height)
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn provided_p1() {
        assert_eq!((21, 0), solve(PROVIDED));
    }
}
