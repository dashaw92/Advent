use aoc::pos::Pos;
use aoc::Plumb;

use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (p1, p2) = solve(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

fn solve(input: impl AsRef<str>) -> (usize, usize) {
    let mut moves: Vec<Delta> = input.plumb();
    let mut rope = Rope::default();
    let mut tail_locs = HashSet::new();

    for d in &mut moves {
        while d.1 != 0 {
            grid(&rope);
            do_move(&mut rope.head, d);
            update_tail(&mut rope);
            tail_locs.insert(rope.tail);
        }
    }

    let p1 = tail_locs.len();
    (p1, 0)
}

fn do_move(pos: &mut Pos<i32>, mv: &mut Delta) {
    match mv.0 {
        Dir::U => *pos -= (0, 1).into(),
        Dir::D => *pos += (0, 1).into(),
        Dir::L => *pos -= (1, 0).into(),
        Dir::R => *pos += (1, 0).into(),
    }

    mv.1 -= 1;
    #[cfg(test)]
    println!("{mv:?}");
}

fn update_tail(rope: &mut Rope) {
    let delta = rope.head - rope.tail;
    match (delta.x(), delta.y()) {
        //the head is "touching" the tail
        (x, y) if x.abs() <= 1 && y.abs() <= 1 => {}
        //diagonals
        (1, -2) => rope.tail += (1, -1).into(),
        (-1, -2) => rope.tail -= (1, 1).into(),
        (1, 2) => rope.tail += (1, 1).into(),
        (-1, 2) => rope.tail -= (1, -1).into(),

        (-2, -1) => rope.tail += (-1, -1).into(),
        (-2, 1) => rope.tail += (-1, 1).into(),
        (2, -1) => rope.tail += (1, -1).into(),
        (2, 1) => rope.tail += (1, 1).into(),

        //left right up down
        (-2, 0) => rope.tail -= (1, 0).into(),
        (2, 0) => rope.tail += (1, 0).into(),
        (0, -2) => rope.tail -= (0, 1).into(),
        (0, 2) => rope.tail += (0, 1).into(),
        _ => {}
    }
}

#[cfg(test)]
fn grid(rope: &Rope) {
    let delta = rope.head - rope.tail;
    let mut grid = vec!['.'; 12 * 10];
    grid[6 * 10 + 5] = 'H';
    grid[(6 + delta.y()) as usize * 10 + (5 + delta.x()) as usize] = 'T';

    for y in 0..12 {
        for x in 0..10 {
            print!("{}", grid[y * 10 + x]);
        }
        println!();
    }
    println!("------------------------");
}

#[cfg(not(test))]
fn grid(_: &Rope) {}

#[derive(Debug)]
struct Rope {
    head: Pos<i32>,
    tail: Pos<i32>,
}

impl Default for Rope {
    fn default() -> Self {
        Rope {
            head: (10, 10).into(),
            tail: (10, 10).into(),
        }
    }
}

#[derive(Debug)]
struct Delta(Dir, i32);

#[derive(Debug)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl FromStr for Delta {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, mag) = s.split_once(' ').unwrap();
        let mag: i32 = mag.parse().unwrap();

        let dir = match dir {
            "U" => Dir::U,
            "D" => Dir::D,
            "L" => Dir::L,
            "R" => Dir::R,
            _ => unreachable!("Bad input!"),
        };

        Ok(Delta(dir, mag))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn provided_p1() {
        assert_eq!((13, 0), solve(PROVIDED));
    }
}
