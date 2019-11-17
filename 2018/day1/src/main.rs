const INPUT: &str = include_str!("../input.txt");

fn main() {
    let parsed: Vec<i32> = INPUT.lines().filter_map(|line| line.parse().ok()).collect();

    println!("The frequency is {}", part1(&parsed));
    println!("The first duplicated frequency is {}", part2(&parsed));
}

//Find the final frequency given the input
//Goes through the input one time.
fn part1(input: &[i32]) -> i32 {
    input.into_iter().sum()
}

//Find the first frequency that has been seen already
//May loop through the input many times before finding the answer
fn part2(input: &[i32]) -> i32 {
    use std::collections::HashSet;

    let mut frequency = 0; //initial frequency
    let mut table = HashSet::new(); //table of seen frequencies
    table.insert(0);

    for amount in input.into_iter().cycle() {
        frequency += amount;

        if !table.insert(frequency) {
            break;
        }
    }

    frequency
}
