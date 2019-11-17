use std::fs::read_to_string;
use std::io::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;

    let mut count = 0;
    let mut count_two = 0;
    let mut map: HashMap<&str, i32> = HashMap::new();
    let mut anagrams: HashMap<String, i32> = HashMap::new();
    input.lines().for_each(|line| {
        line.split(" ").for_each(|word| {
            let mut chars = word.chars().collect::<Vec<_>>();
            chars.sort();
            let joined = chars.iter().collect::<String>();
            *anagrams.entry(joined).or_insert(0) += 1;
            *map.entry(word).or_insert(0) += 1;
        });
        if anagrams.values().filter(|x| **x > 1).count() < 1 {
            count_two += 1;
        }
        if map.values().filter(|x| **x > 1).count() < 1 {
            count += 1;
        }
        anagrams.clear();
        map.clear();
    });

    println!("The answer to part 1 is {}", count);
    println!("The answer to part 2 is {}", count_two);
    Ok(())
}
