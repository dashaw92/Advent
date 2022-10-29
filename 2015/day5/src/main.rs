use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::Result;

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;

    println!("The answer for part 1 is {}", solve_p1(&input));
    println!("The answer for part 2 is {}", solve_p2(&input));
    Ok(())
}

fn check_rules(input: impl AsRef<str>, rules: &[&dyn Fn(&str) -> bool]) -> u32 {
    let mut nice = 0;

    'outer: for line in input.as_ref().lines() {
        for &rule in rules {
            if !rule(line) {
                continue 'outer;
            }
        }

        nice += 1;
    }

    nice
}
fn solve_p1(input: impl AsRef<str>) -> u32 {
    fn nice_substr(input: &str) -> bool {
        !(input.contains("ab")
            || input.contains("cd")
            || input.contains("pq")
            || input.contains("xy"))
    }

    fn has_vowels(input: &str) -> bool {
        const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
        let mut count = 0;
        for ch in input.chars() {
            if VOWELS.contains(&ch) {
                count += 1;
            }
        }

        count >= 3
    }

    fn no_duplicates(input: &str) -> bool {
        let mut dupes = 0;
        let mut peek = input.chars().peekable();
        while let Some(ch) = peek.next() {
            if let Some(&next) = peek.peek() {
                if ch == next {
                    dupes += 1;
                }
            }
        }

        dupes >= 1
    }

    check_rules(input, &[&nice_substr, &has_vowels, &no_duplicates])
}

fn solve_p2(input: impl AsRef<str>) -> u32 {
    fn has_pairs(line: &str) -> bool {
        let chars: Vec<char> = line.chars().collect();
        let pairs: HashSet<String> = chars
            .as_slice()
            .windows(2)
            .map(|chs| chs.iter().copied().collect())
            .collect();

        for pair in pairs {
            if line.match_indices(&pair).count() >= 2 {
                return true;
            }
        }

        false
    }

    fn has_repeat_with_sep(line: &str) -> bool {
        let chars: Vec<char> = line.chars().collect();

        for idx in 0..chars.len() {
            if idx + 2 >= chars.len() {
                break;
            }

            let current = chars[idx];
            let two_from_now = chars[idx + 2];
            if current == two_from_now {
                return true;
            }
        }
        false
    }

    check_rules(input, &[&has_pairs, &has_repeat_with_sep])
}

#[cfg(test)]
mod tests {
    const NAUGHTY: [&str; 2] = ["uurcxstgmygtbstg", "ieodomkazucvgmuy"];

    #[test]
    fn test_naughty_p2() {
        use super::solve_p2;

        for test in NAUGHTY {
            assert_eq!(0, solve_p2(test));
        }
    }
}
