use std::fs::read_to_string;
use std::io::Result;

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    
    let mut paper = 0;
    let mut ribbon = 0;

    for line in input.lines() {
        let mut vals = line.split("x")
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        vals.sort();
        let min = vals[0] * vals[1];
        let squared = (2 * vals[0] * vals[1]) + (2 * vals[1] * vals[2]) + (2 * vals[2] * vals[0]);

        let perimeter = (vals[0] * 2) + (vals[1] * 2);
        let bow = vals[0] * vals[1] * vals[2];

        paper += min + squared;
        ribbon += perimeter + bow;
    }
    println!("The answer for part 1 is {}", paper);
    println!("The answer for part 2 is {}", ribbon);

    Ok(())
}