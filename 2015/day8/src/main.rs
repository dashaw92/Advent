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

fn solve(input: impl AsRef<str>) -> (u32, u32) {
    let mut code_p1 = 0;
    let mut code_p2 = 0;
    let mut mem = 0;
    for line in input.as_ref().lines() {
        let mut tmp = line.len() + 2;
        code_p1 += line.len();
        let chars: Vec<char> = line.chars().collect();

        let mut idx = 0;
        while idx < chars.len() {
            let c = chars[idx];
            match c {
                '\\' if chars[idx + 1] == 'x' => {
                    mem += 1;
                    tmp += 1;
                    idx += 4;
                }
                '\\' => {
                    mem += 1;
                    idx += 2;
                    tmp += 2;
                }
                '"' => {
                    mem += 1;
                    idx += 1;
                    tmp += 1;
                }
                _ => {
                    mem += 1;
                    idx += 1;
                }
            }
        }

        mem -= 2; //start and end quotes
        code_p2 += tmp;
    }

    let p1 = code_p1 - mem;
    let p2 = code_p2 - code_p1;

    (p1 as u32, p2 as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = r#""\x27"
""
"abc"
"aaa\"aaa""#;

    #[test]
    fn test_solver() {
        let (p1, p2) = solve(PROVIDED);
        assert_eq!(p1, 12);
        assert_eq!(p2, 19);
    }
}
