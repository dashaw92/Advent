use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("Part 1: {}", solve_p1(&input));
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

fn solve_p1(input: impl AsRef<str>) -> usize {
    let mut lights = Lights::get();

    for instr in input.as_ref().lines() {
        let (cmd, start, end) = get_cmd(instr);
        for x in start.0..=end.0 {
            for y in start.1..=end.1 {
                lights.set(x, y, cmd);
            }
        }
    }

    lights.count_on()
}

// fn solve_p2(input: impl AsRef<str>) -> usize {
//     0
// }

struct Lights {
    grid: Vec<bool>,
}

impl Lights {
    fn get() -> Self {
        Self {
            grid: vec![false; 1000 * 1000],
        }
    }

    fn set(&mut self, x: usize, y: usize, cmd: Cmd) {
        cmd.run(&mut self.grid[y * 1000 + x]);
    }

    fn count_on(&self) -> usize {
        self.grid.iter().filter(|&&b| b).count()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Cmd {
    On,
    Off,
    Flip,
}
impl Cmd {
    fn run(&self, light: &mut bool) {
        match self {
            Self::On => *light = true,
            Self::Off => *light = false,
            Self::Flip => *light = !*light,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "turn on 0,0 through 999,999";

    #[test]
    fn provided_p1() {
        assert_eq!(1_000_000, solve_p1(PROVIDED));
    }

    // #[test]
    // fn provided_p2() {
    //     assert_eq!(0, solve_p2(PROVIDED));
    // }
}
