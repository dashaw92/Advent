use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
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
    fn from(input: &str) -> Self {
        match input.chars().next().unwrap() {
            'R' => Move::Right(input.get(1..).unwrap().trim().parse().unwrap()),
            'L' => Move::Left(input.get(1..).unwrap().trim().parse().unwrap()),
            _ => Move::None,
        }
    }
}

fn main() {
    let directions: Vec<_> = INPUT.split(", ").map(Move::from).collect();
    let mut visited = HashMap::new();
    let mut rot = Compass::North;
    let mut pos = Point { x: 0, y: 0 };
    let mut found_p2 = false;

    for direction in directions {
        let mut amount = match direction {
            Move::Right(amount) => {
                rot = rot.right();
                amount
            }
            Move::Left(amount) => {
                rot = rot.left();
                amount
            }
            _ => continue,
        };

        while amount > 0 {
            match rot {
                Compass::North => pos.y += 1,
                Compass::South => pos.y -= 1,
                Compass::East => pos.x += 1,
                Compass::West => pos.x -= 1,
            }

            *visited.entry(pos).or_insert(0) += 1;

            if !found_p2 && *visited.get(&pos).unwrap_or(&0) > 1 {
                println!(
                    "Hit an intersection twice! {} blocks away.",
                    pos.x.abs() + pos.y.abs()
                );
                found_p2 = true;
            }

            amount -= 1;
        }
    }

    println!("You are {} blocks away.", pos.x.abs() + pos.y.abs());
}
