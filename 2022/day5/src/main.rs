mod dock;

use std::error::Error;
use std::fs::read_to_string;

use crate::dock::*;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (p1, p2) = solve(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

fn solve(input: impl AsRef<str>) -> (String, usize) {
    let stacks: Vec<&str> = input
        .as_ref()
        .lines()
        .take_while(|line| !line.is_empty())
        .collect();
    let mut dock = Dock::new(stacks);

    input
        .as_ref()
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .filter_map(|mv| mv.parse::<Move>().ok())
        .for_each(|mv| dock.run(&mv));

    (dock.get_p1_output(), 0)
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn provided_p1() {
        assert_eq!(("CMZ".into(), 0), solve(PROVIDED));
    }
}
