#![allow(unused)]

use std::collections::HashSet;
use std::str::FromStr;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub struct Point {
    x: isize,
    y: isize,
}

#[derive(Copy, Clone, Debug)]
pub enum Move {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

impl FromStr for Move {
    type Err = ();

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        let prefix = val.chars().nth(0).unwrap();
        let magnitude = val[1..].parse().unwrap();

        match prefix {
            'R' => Ok(Self::Right(magnitude)),
            'L' => Ok(Self::Left(magnitude)),
            'U' => Ok(Self::Up(magnitude)),
            'D' => Ok(Self::Down(magnitude)),
            _ => Err(()),
        }
    }
}

impl Move {
    fn get_x(&self) -> isize {
        match *self {
            Self::Left(x) => -x,
            Self::Right(x) => x,
            _ => 0,
        }
    }

    fn get_y(&self) -> isize {
        match *self {
            Self::Up(y) => -y,
            Self::Down(y) => y,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Wire {
    x: isize,
    y: isize,
    locations: HashSet<Point>,
}

impl Wire {
    pub fn new() -> Self {
        Wire {
            x: 0,
            y: 0,
            locations: HashSet::new(),
        }
    }

    pub fn take_move(&mut self, mov: Move) {
        let min_x = isize::min(self.x, self.x + mov.get_x());
        let max_x = isize::max(self.x, self.x + mov.get_x());

        let min_y = isize::min(self.y, self.y + mov.get_y());
        let max_y = isize::max(self.y, self.y + mov.get_y());

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                self.locations.insert(Point { x, y });
            }
        }

        self.x += mov.get_x();
        self.y += mov.get_y();
    }

    pub fn find_matches(wire1: Wire, wire2: Wire) -> Vec<Point> {
        let mut matches = Vec::new();

        for pos1 in &wire1.locations {
            for pos2 in &wire2.locations {
                if pos1 == pos2 {
                    matches.push(*pos1);
                }
            }
        }

        matches
    }
}
