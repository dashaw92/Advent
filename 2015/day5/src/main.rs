use std::fs::read_to_string;
use std::io::Result;

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;

    println!("The answer for part 1 is {}", solve_p1(&input));
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
