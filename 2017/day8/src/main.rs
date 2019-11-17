use std::fs::read_to_string;
use std::io::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let mut running_max = i32::min_value();
    let mut registers: HashMap<&str, i32> = HashMap::new();

    for line in input.lines() {
        let words: Vec<&str> = line.split("if ").collect();
        let test: Vec<&str> = words[1].split(' ').collect();
        let (reg, op, val): (&str, &str, i32) = (test[0], test[1], test[2].parse().unwrap());

        if run_test(*registers.entry(reg).or_insert(0), val, op) {
            let left: Vec<&str> = words[0].split(' ').collect();
            let (reg, op, val): (&str, &str, i32) = (left[0], left[1], left[2].parse().unwrap());

            if op == "dec" {
                *registers.entry(reg).or_insert(0) -= val;
            } else {
                *registers.entry(reg).or_insert(0) += val;
            }
            let cur_max = registers.values().max().unwrap();
            if *cur_max > running_max {
                running_max = *cur_max;
            }
        }
    }

    let max = registers.values().max().unwrap();
    println!("The answer to part 1 is {}", max);
    println!("The answer to part 2 is {}", running_max);
    Ok(())
}

fn run_test(left: i32, right: i32, oper: &str) -> bool {
    match oper {
        ">" => left > right,
        "<" => left < right,
        ">=" => left >= right,
        "<=" => left <= right,
        "!=" => left != right,
        "==" => left == right,
        _ => false,
    }
}
