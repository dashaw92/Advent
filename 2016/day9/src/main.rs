use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (p1, p2) = solve(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

fn solve(input: impl AsRef<str>) -> (usize, usize) {
    let mut buf = Vec::with_capacity(input.as_ref().len());
    let input = input.as_ref();
    let chars: Vec<char> = input.chars().collect();

    // Simple strategy: Iterate over every char in the input. The only special case that needs to be
    // considered is the start of a marker. Note that the input can contain '(' as a literal character,
    // but the way markers are consumed from the iteration is greedy, so no extra consideration is needed
    // in this case.
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            //This is the start of a marker
            '(' => {
                //We need to find the end of the marker because markers
                //can have variable widths based on the digits used,
                //i.e "(10x50)" vs "(1x2)". Just mark the current position
                //and increase it until we find the first ')'. This will
                //give us the range in the string from which to extract
                //the complete marker.
                let mut end = i;
                while end < chars.len() && chars[end] != ')' {
                    end += 1;
                }

                //Ensure we found a marker. This will panic if the marker does not
                //finish. Needs testing.
                match &chars[i..=end] {
                    //x: The whole marker with '(' and ')'
                    //inner: Everything between the parens
                    x @ &['(', ref inner @ .., ')'] => {
                        //Build the string for parsing into ints
                        let (length, repeat) = {
                            let marker: String = inner.iter().collect();
                            let (length, repeat) = marker.split_once('x').unwrap();
                            let length: usize = length.parse().unwrap();
                            let repeat: usize = repeat.parse().unwrap();
                            (length, repeat)
                        };

                        if end + length >= chars.len() {
                            panic!("{} + {} >= {} (chars.len())", end, length, chars.len());
                        }

                        let compressed = &chars[end + 1..=end + length];
                        (0..repeat).for_each(|_| {
                            buf.extend_from_slice(compressed);
                        });

                        //Manually increment `i` to continue from where we left off.
                        //The marker has been completely used by this point, so we can
                        //continue like normal after this.
                        i += x.len() + length;
                        continue;
                    }
                    //The marker is not valid. The input *shouldn't* trigger this if my code
                    //is correct. *fingers crossed*
                    _ => {
                        panic!("Malformed marker in input.");
                    }
                }
            }
            //This is not a marker, so just add it to the final output buffer.
            c => buf.push(c),
        }
        i += 1;
    }

    let len_no_blanks = buf.iter().filter(|c| !c.is_whitespace()).count();
    (len_no_blanks, 0)
}

#[cfg(test)]
mod test {

    use super::*;

    //Should become "ABBBBBC" (7) once the solver is finished.
    const PROVIDED: &str = "A(1x5)BC";

    #[test]
    fn provided_p1() {
        assert_eq!((7, 0), solve(PROVIDED));
    }
}
