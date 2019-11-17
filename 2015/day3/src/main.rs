use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Result;

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;

    println!("The answer to part 1 is {}", solve(&input, false));
    println!("The answer to part 2 is {}", solve(&input, true));

    Ok(())
}

fn solve(input: &String, part2: bool) -> usize {
    let mut real = (0, 0);
    let mut robo = (0, 0);
    let mut houses: HashMap<(i32, i32), i32> = HashMap::new();
    houses.insert((0, 0), 1);

    for (i, ch) in input.chars().enumerate() {
        let santa = if !part2 || i % 2 == 0 {
            &mut real
        } else {
            &mut robo
        };
        match ch {
            '>' => *santa = (santa.0 + 1, santa.1),
            '<' => *santa = (santa.0 - 1, santa.1),
            '^' => *santa = (santa.0, santa.1 + 1),
            'v' => *santa = (santa.0, santa.1 - 1),
            _ => (),
        }
        *houses.entry(*santa).or_insert(0) += 1;
    }

    houses.len()
}
