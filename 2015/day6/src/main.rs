use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (p1, p2) = solve(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

type Pair = (usize, usize);

fn parse_pair(input: &str) -> Pair {
    input
        .split_once(',')
        .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
        .unwrap()
}

fn get_pairs(input: &str, prefix: &str) -> (Pair, Pair) {
    let (start_pos, end_pos) = input
        .strip_prefix(prefix)
        .and_then(|s| s.split_once(" through "))
        .unwrap();
    let start_pos: Pair = parse_pair(start_pos);
    let end_pos: Pair = parse_pair(end_pos);

    (start_pos, end_pos)
}

fn get_cmd(input: &str) -> (Cmd, Pair, Pair) {
    if input.starts_with("turn on ") {
        let (start, end) = get_pairs(input, "turn on ");
        (Cmd::On, start, end)
    } else if input.starts_with("turn off ") {
        let (start, end) = get_pairs(input, "turn off ");
        (Cmd::Off, start, end)
    } else {
        let (start, end) = get_pairs(input, "toggle ");
        (Cmd::Flip, start, end)
    }
}

fn solve(input: impl AsRef<str>) -> (i32, i32) {
    let mut lights_p1 = Lights::get();
    let mut lights_p2 = Lights::get();

    for instr in input.as_ref().lines() {
        let (cmd, start, end) = get_cmd(instr);
        for x in start.0..=end.0 {
            for y in start.1..=end.1 {
                lights_p1.set(x, y, cmd, true);
                lights_p2.set(x, y, cmd, false);
            }
        }
    }

    let p1 = lights_p1.count_on(true);
    let p2 = lights_p2.count_on(false);

    (p1, p2)
}

struct Lights {
    grid: Vec<i32>,
}

impl Lights {
    fn get() -> Self {
        Self {
            grid: vec![0; 1000 * 1000],
        }
    }

    fn set(&mut self, x: usize, y: usize, cmd: Cmd, part1: bool) {
        cmd.run(&mut self.grid[y * 1000 + x], part1);
    }

    fn count_on(&self, part1: bool) -> i32 {
        if part1 {
            return self.grid.iter().filter(|&&val| val > 0).count() as i32;
        }

        self.grid.iter().sum::<i32>()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Cmd {
    On,
    Off,
    Flip,
}

impl Cmd {
    fn run(&self, light: &mut i32, part1: bool) {
        match self {
            Self::On => {
                if part1 {
                    *light = 1;
                } else {
                    *light += 1;
                }
            }
            Self::Off => {
                if part1 {
                    *light = 0;
                } else {
                    *light = 0.max(*light - 1);
                }
            }
            Self::Flip => {
                if part1 {
                    if *light == 0 {
                        *light = 1;
                    } else {
                        *light = 0;
                    }
                } else {
                    *light += 2;
                }
            }
        }
    }
}
