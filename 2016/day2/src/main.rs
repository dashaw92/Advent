use std::error::Error;
use std::fs::read_to_string;

type Pos = (i32, i32);

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (p1, p2) = solve(&input);
    println!("Key code for p1 is {}", p1);
    println!("Key code for p2 is {}", p2);
    Ok(())
}

fn solve(input: impl AsRef<str>) -> (String, String) {
    let p1 = solve_p1(input.as_ref());
    let p2 = solve_p2(input.as_ref());

    (p1, p2)
}

fn solve_p1(input: impl AsRef<str>) -> String {
    let pad = Keypad::p1();
    let mut pos = p1_origin();

    input
        .as_ref()
        .lines()
        .map(|l| do_line(&pad, &mut pos, l))
        .collect()
}

fn solve_p2(input: impl AsRef<str>) -> String {
    let pad = Keypad::p2();
    let mut pos = p2_origin();

    input
        .as_ref()
        .lines()
        .map(|l| do_line(&pad, &mut pos, l))
        .collect()
}

fn p1_origin() -> Pos {
    (1, 1)
}

fn p2_origin() -> Pos {
    (0, 3)
}

fn do_step(pad: &Keypad, dir: char, pos: &mut Pos) -> Pos {
    let mut my_pos = *pos;
    match dir {
        'U' => my_pos.1 -= 1,
        'D' => my_pos.1 += 1,
        'L' => my_pos.0 -= 1,
        'R' => my_pos.0 += 1,
        _ => unreachable!(),
    }

    my_pos = (
        0.max((pad.size - 1).min(my_pos.0)),
        0.max((pad.size - 1).min(my_pos.1)),
    );

    if pad.key_at(my_pos) == ' ' {
        //p2 has blank spots marked as ' ',
        //and we can't move to them. This stops
        //before those edges.
        return *pos;
    }

    my_pos
}

fn do_line(pad: &Keypad, start: &mut Pos, line: &str) -> char {
    for ch in line.chars() {
        *start = do_step(pad, ch, start);
    }

    pad.press(*start)
}

struct Keypad {
    keys: Vec<char>,
    size: i32,
}

impl Keypad {
    fn p1() -> Self {
        Self {
            keys: vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'],
            size: 3,
        }
    }

    fn p2() -> Self {
        Self {
            keys: vec![
                ' ', ' ', '1', ' ', ' ', ' ', '2', '3', '4', ' ', '5', '6', '7', '8', '9', ' ',
                'A', 'B', 'C', ' ', ' ', ' ', 'D', ' ', ' ',
            ],
            size: 5,
        }
    }

    fn key_at(&self, pos: Pos) -> char {
        let idx = (pos.1 * self.size + pos.0) as usize;
        self.keys[idx]
    }

    fn press(&self, pos: Pos) -> char {
        let ch = self.key_at(pos);
        if ch == ' ' {
            panic!("Your code is buggy!");
        }

        ch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = "ULL
RRDDD
LURDL
UUUUD";

    #[test]
    fn test() {
        assert_eq!("1985", &solve_p1(PROVIDED));
        assert_eq!("5DB3", &solve_p2(PROVIDED));
    }
}
