#![allow(dead_code)]

use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

use self::D::*;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("Distance for part 1: {}", solve_p1(&input));
    Ok(())
}

fn simulate(steps: &str) -> Pos {
    let pos = steps
        .split(',')
        .filter_map(|s| s.parse().ok())
        .fold(Pos::origin(), |pos, dir| pos.apply(dir));
    pos
}

fn solve_p1(input: impl AsRef<str>) -> i32 {
    let pos = simulate(input.as_ref());
    pos.distance(&Pos::origin())
}

//Doubled coords (double y)
#[derive(Debug, Clone, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}
impl Pos {
    fn origin() -> Self {
        Pos { x: 0, y: 0 }
    }

    fn at((x, y): (i32, i32)) -> Self {
        Pos { x, y }
    }

    fn apply(&self, dir: D) -> Self {
        Pos::at(match dir {
            N => (self.x, self.y - 2),
            S => (self.x, self.y + 2),
            NE => (self.x + 1, self.y - 1),
            SE => (self.x + 1, self.y + 1),
            SW => (self.x - 1, self.y + 1),
            NW => (self.x - 1, self.y - 1),
        })
    }

    //http://ondras.github.io/rot.js/manual/#hex/indexing @ #3: Double width
    fn distance(&self, other: &Pos) -> i32 {
        let dy = self.y.abs_diff(other.y) as i32;
        let dx = self.x.abs_diff(other.x) as i32;

        dx + 0i32.max((dy - dx) / 2)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum D {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}
impl FromStr for D {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().trim() {
            "n" => N,
            "ne" => NE,
            "se" => SE,
            "s" => S,
            "sw" => SW,
            "nw" => NW,
            _ => return Err(()),
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    //ensure that the coordinate system holds the constraint (col + row) % 2 == 0
    #[test]
    fn test_coordinates() {
        let dirs: Vec<D> = "se,sw,se,sw,sw"
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();
        let mut pos = Pos::origin();

        for d in dirs {
            pos.apply(d);
            assert_eq!(0, (pos.x + pos.y) % 2)
        }
    }

    #[test]
    fn coordinate_distance() {
        fn simulate(steps: &str, expected: i32) {
            let dirs: Vec<D> = steps.split(',').filter_map(|s| s.parse().ok()).collect();
            let mut pos = Pos::origin();

            dirs.into_iter().for_each(|d| pos.apply(d));
            assert_eq!(expected, pos.distance(&Pos::origin()));
        }

        simulate("ne,ne,ne", 3);
        simulate("ne,ne,sw,sw", 0);
        simulate("ne,ne,s,s", 2);
        simulate("se,sw,se,sw,sw", 3);
    }
}
