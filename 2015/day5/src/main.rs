use std::fs::read_to_string;
use std::io::Result;

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let mut count = 0;

    for line in input.lines() {
        if line.trim() == "" || !nice_substr(line) || line.matches(is_vowel).count() < 3 {
            continue;
        }
        let mut iter = line.chars().peekable();
        while let Some(current) = iter.next() {
            let next = iter.peek().unwrap_or(&'0');
            if current == *next {
                count += 1;
                break;
            }
        }
    }

    println!("The answer for part 1 is {}", count);
    Ok(())
}

/// True means it's nice
fn nice_substr(input: &str) -> bool {
    !(input.contains("ab") || input.contains("cd") || input.contains("pq") || input.contains("xy"))
}

fn is_vowel(input: char) -> bool {
    match input {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}
