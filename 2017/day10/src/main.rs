use std::fs::read_to_string;
use std::io::Result;

fn main() -> Result<()> {
    let input: Vec<i32> = read_to_string("input.txt")?
        .split(",")
        .filter_map(|val| val.parse::<i32>().ok())
        .collect();

    let mut seq: Vec<i32> = (0..256).collect();

    twist(&mut seq, &input);
    println!("Day 1: {}", seq[0] * seq[1]);
    Ok(())
}

fn twist(seq: &mut Vec<i32>, twists: &[i32]) {
    let mut skip = 0;
    let mut cursor = 0usize;

    for &len in twists {
        let end_actual = len as usize + cursor;
        
        let mut current = Vec::new();
        for idx in cursor..end_actual {
            let idx_wrapped = idx % seq.len();
            current.push(seq[idx_wrapped]);
        }

        current.reverse();
        for val in current {
            seq[cursor] = val;
            cursor += 1;
            cursor %= seq.len();
        }

        cursor += skip;
        skip += 1;
    }

}