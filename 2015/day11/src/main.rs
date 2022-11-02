use std::{collections::HashSet, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::env::args().nth(1).unwrap();

    println!("Part 1: {}", solve_p1(&input));
    Ok(())
}

//must have 3 chars in a row that increase
//"abc" passes
//"asdf" fails
fn has_straight(input: &str) -> bool {
    input
        .as_bytes()
        .windows(3)
        .any(|slice| slice[0] == slice[1] - 1 && slice[0] == slice[2] - 2)
}

//"these chars are easy to confuse with others: 'i', 'o', 'l'"
fn no_homoglyphs(input: &str) -> bool {
    let bad: [char; 3] = ['i', 'o', 'l'];
    !input.chars().any(|ch| bad.contains(&ch))
}

//must contain two unique pairs of non-overlapping letters
fn has_pairs(input: &str) -> bool {
    let pairs: HashSet<_> = input
        .as_bytes()
        .windows(2)
        .filter(|slice| slice.len() > 1 && slice[0] == slice[1])
        .map(|slice| slice[0])
        .collect();
    pairs.len() > 1
}

fn is_valid(input: &str) -> bool {
    has_straight(input) && no_homoglyphs(input) && has_pairs(input)
}

fn solve_p1(input: impl AsRef<str>) -> String {
    let mut current = input.as_ref().to_string();
    while !is_valid(current.as_str()) {
        current = inc_pw(current);
    }
    current
}

fn inc_pw(pw: impl AsRef<str>) -> String {
    let mut chars: Vec<char> = pw.as_ref().chars().collect();

    let mut carry = 1;
    let mut idx = (chars.len() - 1) as i32;
    while carry > 0 {
        if idx < 0 {
            chars.push('a');
            break;
        }

        let mut ch = chars[idx as usize];
        ch = ((ch as u8) + carry) as char;
        if ch > 'z' {
            chars[idx as usize] = 'a';
            carry = 1;
            idx -= 1;
        } else {
            chars[idx as usize] = ch;
            carry = 0;
        }
    }

    chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_provided() {
        assert_eq!("abcdffaa", &solve_p1("abcdefgh"));
        assert_eq!("ghjaabcc", &solve_p1("ghijklmn"));
    }
}
