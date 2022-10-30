use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("Part 1: {}", solve_p1(&input));
    Ok(())
}

fn solve_p1(input: impl AsRef<str>) -> u32 {
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

    let mem = runes
        .iter()
        .fold(0, |acc, rune| acc + rune.mem_repr_count());
    let code_repr = runes
        .iter()
        .filter(|rune| !matches!(rune, Rune::Lit('"')))
        .count();
    mem - code_repr as u32
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
mod test {

    use super::*;

    const PROVIDED: &str = r#"""
"abc"
"aaa\"aaa"
"\x27"#;

    #[test]
    fn provided_p1() {
        assert_eq!(5, solve_p1("\"\\x27\""));
    }
}
