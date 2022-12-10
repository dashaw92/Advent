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
    let p1 = run(&input, 2);
    let p2 = run(&input, 10);
    (p1, p2)
}

fn run(input: impl AsRef<str>, size: usize) -> usize {
    let mut moves: Vec<Delta> = input.plumb();
    let mut tail_locs = HashSet::new();
    let mut rope = Rope::new(size);

    for d in &mut moves {
        while d.1 != 0 {
            do_move(&mut rope.body[0], d);
            update_tail(&mut rope);
            d.1 -= 1;

            tail_locs.insert(*rope.body.last().unwrap());
        }
    }

    tail_locs.len()
}

fn do_move(pos: &mut Pos<i32>, mv: &Delta) {
    let delta = match mv.0 {
        Dir::U => (0, -1).into(),
        Dir::D => (0, 1).into(),
        Dir::L => (-1, 0).into(),
        Dir::R => (1, 0).into(),
    };

    *pos += delta;
}

fn update_tail(rope: &mut Rope) {
    for i in 0..rope.body.len() {
        if i + 1 == rope.body.len() {
            break;
        }

        let head = rope.body[i];
        let tail = &mut rope.body[i + 1];

        let delta = head - *tail;
        match (delta.x(), delta.y()) {
            //the head is "touching" the tail
            (x, y) if x.abs() <= 1 && y.abs() <= 1 => {}
            //diagonals
            (1, -2) => *tail += (1, -1).into(),
            (-1, -2) => *tail -= (1, 1).into(),
            (1, 2) => *tail += (1, 1).into(),
            (-1, 2) => *tail -= (1, -1).into(),

            (-2, -1) => *tail += (-1, -1).into(),
            (-2, 1) => *tail += (-1, 1).into(),
            (2, -1) => *tail += (1, -1).into(),
            (2, 1) => *tail += (1, 1).into(),

            (2, -2) => *tail += (1, -1).into(),
            (2, 2) => *tail += (1, 1).into(),
            (-2, -2) => *tail += (-1, -1).into(),
            (-2, 2) => *tail += (-1, 1).into(),

            //left right up down
            (-2, 0) => *tail -= (1, 0).into(),
            (2, 0) => *tail += (1, 0).into(),
            (0, -2) => *tail -= (0, 1).into(),
            (0, 2) => *tail += (0, 1).into(),
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
struct Rope {
    body: Vec<Pos<i32>>,
}

impl Rope {
    fn new(size: usize) -> Self {
        Rope {
            body: vec![(0, 0).into(); size],
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

    const PROVIDED_P2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn provided_p1() {
        assert_eq!(13, run(PROVIDED, 2));
    }

    #[test]
    fn provided_p2() {
        assert_eq!(36, run(PROVIDED_P2, 10));
    }
}
