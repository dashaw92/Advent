use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("Part 1: {}", solve_p1(&input));
    Ok(())
}

fn score(c: char) -> usize {
    match c {
        ')' | '(' => 3,
        ']' | '[' => 57,
        '}' | '{' => 1197,
        '>' | '<' => 25137,
        _ => 0,
    }
}

fn solve_p1(input: impl AsRef<str>) -> usize {
    fn o(c: char) -> char {
        match c {
            ')' => '(',
            ']' => '[',
            '}' => '{',
            '>' => '<',
            _ => panic!("unknown char {}", c),
        }
    }

    let input: Vec<&str> = input.as_ref()
        .lines()
        .collect();
    let mut ans = 0;

    for nav in input {
        let mut stack: Vec<char> = Vec::new();
        let chars = nav.chars();
        for ch in chars {
            match ch {
                '(' | '[' | '{' | '<' => stack.push(ch),
                close => {
                    let last = match stack.pop() {
                        Some(last) => last,
                        None => {
                            //ignore
                            continue;
                        }
                    };

                    if o(close) != last {
                        ans += score(close);
                    }
                }
            }
        }
    }

    ans
}

fn solve_p2(input: impl  AsRef<str>) -> usize {
    0
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn provided_p1() {
        assert_eq!(26397, solve_p1(PROVIDED));
    }

    #[test]
    fn provided_p2() {
        assert_eq!(0, solve_p2(PROVIDED));
    }
}