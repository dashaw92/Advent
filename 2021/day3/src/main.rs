use std::error::Error;
use std::fs::read_to_string;

mod util;

fn main() -> Result<(), Box<dyn Error>>{
    let input: Vec<usize> = read_to_string("input.txt")?
        .lines()
        .filter_map(|x| usize::from_str_radix(x, 2).ok())
        .collect();

    println!("Part 1: {}", solve(&input));
    Ok(())
}

fn solve(input: &[usize]) -> usize {
    use util::*;
    //every index represents a bit starting from LSB.
    //index 0 = 1st bit
    //value = number of times it was 1 in the entire list of input
    let mut gamma_bits = [0; NUM_BITS];

    input.iter().for_each(|&digit| run_for_num(digit, &mut gamma_bits));

    //convert the decimal array to binary
    normalize(input.len(), &mut gamma_bits, true);

    let gamma = to_decimal(&gamma_bits);
    let epsilon = loop {
        let mut epsilon = 0;
        let mut gamma = gamma;
        let mut idx = 0;
        while gamma > 0 {
            let bit = gamma & (1 << idx);
            epsilon |= match bit {
                0 => 1 << idx,
                _ => 0,
            };

            gamma ^= bit;
            idx += 1;
        }

        break epsilon
    };

    gamma * epsilon
}

#[cfg(test)]
mod test {
    use super::*;

    const PROVIDED: [usize; 12] = [
        0b00100,
        0b11110,
        0b10110,
        0b10111,
        0b10101,
        0b01111,
        0b00111,
        0b11100,
        0b10000,
        0b11001,
        0b00010,
        0b01010,
    ];

    #[test]
    fn provided_p1() {
        assert_eq!(198, solve(&PROVIDED))
    }
}