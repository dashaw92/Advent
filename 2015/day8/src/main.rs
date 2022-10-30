use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;
    // let input = r#""\x27""#;

    let (p1, p2) = solve(&input);
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    Ok(())
}

fn to_runes(input: impl AsRef<str>) -> Vec<Rune> {
    let mut runes = Vec::new();
    let chars: Vec<char> = input.as_ref().chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let current = chars[i];
        //assume \n is not considered part of this
        if current == '\n' {
            i += 1;
            continue;
        }

        if i + 4 < chars.len() && current == '\\' && chars[i + 1] == 'x' {
            let hex = &chars[i + 2..=i + 3];
            runes.push(Rune::Hex(hex[0], hex[1]));
            i += 4;
        } else if i + 2 < chars.len() && chars[i] == '\\' {
            runes.push(Rune::Esc(chars[i + 1]));
            i += 2;
        } else {
            runes.push(Rune::Lit(current));
            i += 1;
        }
    }

    runes
}

fn count(runes: &[Rune]) -> (u32, u32) {
    let mem = runes
        .iter()
        .fold(0, |acc, rune| acc + rune.mem_repr_count());
    let code_repr = runes
        .iter()
        .filter(|rune| !matches!(rune, Rune::Lit('"')))
        .count() as u32;

    (mem, code_repr)
}

fn solve(input: impl AsRef<str>) -> (u32, u32) {
    let runes = to_runes(input);

    let (mem, code_repr) = count(&runes);
    let p1 = mem - code_repr as u32;
    let p2 = solve_p2(runes, mem);

    (p1, p2)
}

fn solve_p2(runes: Vec<Rune>, mem_before: u32) -> u32 {
    let mut output = 0;
    for rune in &runes {
        match rune {
            Rune::Lit('"') => output += 2,
            Rune::Esc(_) => output += 4,
            Rune::Hex(..) => output += 5,
            Rune::Lit(_) => output += 1,
        }
    }

    output += 2; //quotes
    output - mem_before
}

#[derive(Debug)]
enum Rune {
    Lit(char),
    Hex(char, char),
    Esc(char),
}

impl Rune {
    fn mem_repr_count(&self) -> u32 {
        match self {
            Self::Lit(..) => 1,
            Self::Hex(..) => 4,
            Self::Esc(..) => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = r#""\x27""#;

    #[test]
    fn test_solver_p2() {
        println!("{:?}", solve(PROVIDED));
    }
}
