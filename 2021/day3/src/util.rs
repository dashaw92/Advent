pub const NUM_BITS: usize = 12;

//process the bits of a number
pub fn run_for_num(input: usize, bits: &mut [usize; NUM_BITS]) {
    const MASK: usize = 1;
    (0..NUM_BITS).for_each(|idx| {
        let bit = match input & (MASK << idx) {
            0 => 0,
            _ => 1,
        };
        bits[idx] += bit;
    });
}

//if powers[n] > len / 2, powers[n] => 1
//else powers[n] => 0
pub fn normalize(len: usize, bits: &mut [usize; NUM_BITS], most_common: bool) {
    (0..bits.len()).for_each(|idx| {
        bits[idx] = match (bits[idx] >= len / 2, most_common) {
            (true, true) => 1, //more common and we want most common
            (true, false) => 0, //less common and we want inverse
            (false, true) => 0, //more common and we want most common
            (false, false) => 1, //less common and we want inverse
        };
    });
}

//[1,1,0,1] => 0b1101
pub fn to_decimal(bits: &[usize; NUM_BITS]) -> usize {
    bits.iter()
        .enumerate()
        .fold(0, |acc, (idx, bit)| acc | (bit << idx))
}