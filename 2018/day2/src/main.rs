const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("The checksum for part 1 is {}", part1());
    println!("Part 2 (common characters): {}", String::from_utf8_lossy(&part2()));
}

//Iterates through the input, and finds the number of
//lines that contain exactly two of the same characters
//and lines that contain exactly three of the same characters
//counting one line per condition only once.
//aabbbccddd = 1 set of two chars, and 1 set of three chars.
fn part1() -> i32 {
    use std::collections::HashMap;

    let mut twos = 0;
    let mut threes = 0;

    for line in INPUT.lines() {
        let mut table = HashMap::new();
        let mut found_two = false; //Avoid counting a line for each condition more than once
        let mut found_three = false;

        for ch in line.chars() {
            *table.entry(ch).or_insert(0) += 1;
        }

        table
            .into_iter()
            .filter(|(_, v)| *v >= 2) //Only count chars that appeared more than 1 time
            .for_each(|(_, v)| {
                //Update `twos` and `threes` accordingly
                if v == 2 && !found_two {
                    found_two = true;
                    twos += 1;
                } else if v == 3 && !found_three {
                    found_three = true;
                    threes += 1;
                }
            });
    }

    twos * threes //The basic checksum
}

//Find the two strings in the input whose levenshtein distance is 1.
//Then, find the common characters between both strings
fn part2() -> Vec<u8> {
    use levenshtein::levenshtein;

    let copy_input = INPUT.clone();
    let mut first = "";
    let mut second = "";

    'outer: for line in INPUT.lines() {
        for line2 in copy_input.lines() {
            if levenshtein(line, line2) == 1 {
                first = line;
                second = line2;
                break 'outer;
            }
        }
    }

    //sanity
    if first == "" || second == "" {
        panic!("first or second are both empty!")
    }

    first.bytes()
        .zip(second.bytes())
        .filter(|(ch1, ch2)| ch1 == ch2)
        .map(|chars| chars.0)
        .collect()
}
