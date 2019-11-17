use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
enum Compass {
    North,
    East,
    South,
    West,
}

impl Compass {
    fn right(&self) -> Self {
        match self {
            Compass::North => Compass::East,
            Compass::East => Compass::South,
            Compass::South => Compass::West,
            Compass::West => Compass::North,
        }
    }

    fn left(&self) -> Self {
        match self {
            Compass::North => Compass::West,
            Compass::East => Compass::North,
            Compass::South => Compass::East,
            Compass::West => Compass::South,
        }
    }
}

enum Move {
    Right(i32),
    Left(i32),
    None,
}

impl Move {

    //Left unsafe so any errors just crash and don't let the program run.
    //Saves me time assuming my program is correct if it runs.
    fn from(input: &str) -> Self {
        match input.chars().nth(0).unwrap() {
            'R' => Move::Right(input.get(1..).unwrap().trim().parse().unwrap()),
            'L' => Move::Left(input.get(1..).unwrap().trim().parse().unwrap()),
            _   => Move::None,
        }
    }
}

fn main() {
    let directions: Vec<_> = "R5, L5, R5, R3".split(", ").map(Move::from).collect();
    let mut visited = HashMap::new();
    let mut rot = Compass::North;
    let mut pos = Point { x: 0, y: 0 };

    for direction in directions {
        let amount = match direction {
            Move::Right(amount) => {
                rot = rot.right();
                amount
            },
            Move::Left(amount) => {
                rot = rot.left();
                amount
            },
            _ => continue,
        };

        match rot {
            Compass::North => pos.y += amount,
            Compass::South => pos.y -= amount,
            Compass::East => pos.x += amount,
            Compass::West => pos.x -= amount, 
        }

        if visited.contains_key(&pos) {
            println!("Hit an intersection twice! {} blocks away.", i32::abs(pos.x - pos.y));
        }
        *visited.entry(pos).or_insert(0) += 1;
    }

    println!("You are {} blocks away.", i32::abs(pos.x - pos.y));
}