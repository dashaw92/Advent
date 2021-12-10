use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("Part 1: {}", solve_p1(&input));
    println!("Part 2: {}", solve_p2(&input));
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

fn o(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("unknown char {}", c),
    }
}

fn c(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("unknown char {}", c),
    }
}

fn solve_p1(input: impl AsRef<str>) -> usize {
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
    let input: Vec<&str> = input.as_ref()
        .lines()
        .collect();
    let mut scores = Vec::new();

    'outer: for nav in input {
        let mut stack: Vec<char> = Vec::new();
        let chars = nav.chars();
        for ch in chars {
            match ch {
                '(' | '[' | '{' | '<' => stack.push(ch),
                close => {
                    let last = stack.pop().unwrap();
                    let expected = o(close);
                    if last != expected {
                        continue 'outer; //ignore corrupted lines
                    }
                }
            }
        }

        //at this point, stack contains only open brackets
        scores.push(stack.into_iter().rev().map(c).fold(0, |acc, ch| {
            acc * 5 + match ch {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
            }
        }));
    }

    scores.sort();
    scores[scores.len() / 2]
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
        assert_eq!(288957, solve_p2(PROVIDED));
    }
}