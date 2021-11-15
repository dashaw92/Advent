use std::fs::read_to_string;
use std::io::Result;

fn main() -> Result<()> {
    day1().and(day2())
}

fn day1() -> Result<()> {
    let input: Vec<u32> = read_to_string("input.txt")?
        .split(",")
        .filter_map(|val| val.parse::<u32>().ok())
        .collect();

    let mut seq: Vec<u32> = (0..256).collect();
    let mut params = Params::new();

    twist(&mut seq, &input, &mut params);
    println!("Day 1: {}", seq[0] * seq[1]);

    Ok(())
}

fn day2() -> Result<()> {
    let mut input: Vec<u32> = read_to_string("input.txt")?
        .chars()
        .map(Into::into)
        .collect();

    input.extend_from_slice(&[17, 31, 73, 47, 23]);

    let mut seq: Vec<u32> = (0..256).collect();

    let mut params = Params::new();
    for _iter in 0..64 {
        twist(&mut seq, &input, &mut params);
    }

    let mut dense: [u32; 16] = [0; 16];
    let chunks: Vec<&[u32]> = seq.chunks(16).collect();
    for idx in 0..chunks.len() {
        dense[idx] = chunks[idx].iter()
            .fold(0, |acc, val| acc ^ val);
    }

    let hash = format!("{:x?}", dense).replace("[", "").replace("]", "").replace(", ", "");
    println!("{}", hash);
    Ok(())
}

fn twist(seq: &mut Vec<u32>, twists: &[u32], params: &mut Params) {
    for &len in twists {
        let end_actual = len as usize + params.cursor;
        
        let mut current = Vec::new();
        for idx in params.cursor..end_actual {
            let idx_wrapped = idx % seq.len();
            current.push(seq[idx_wrapped]);
        }

        current.reverse();
        for val in current {
            params.cursor %= seq.len();
            seq[params.cursor] = val;
            params.cursor += 1;
        }

        params.finalize_iteration()
    }

}

struct Params {
    cursor: usize,
    skip: usize,
}

impl Params {
    
    fn new() -> Self {
        Params { cursor: 0, skip: 0 }
    }

    fn finalize_iteration(&mut self) {
        self.cursor += self.skip;
        self.skip += 1;
    }

}