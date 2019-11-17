use std::fs::read_to_string;
use std::io::Result;

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;

    let mut score = 0;
    let mut inside = 0; //What level of "nest" we're on
    let mut garbage = false; //Are we in a garbage block?
    let mut garbage_chars = 0;
    let mut skip_ch = false; //Should we skip the next char or not? ("!")
    for ch in input.chars() {
        if skip_ch {
            if garbage {
                //Account for skipped chars not being counted for part 2.
                garbage_chars -= 1;
            }
            skip_ch = false;
            continue;
        }
        if garbage && ch != '>' {
            garbage_chars += 1;
        }
        match ch {
            '{' if !garbage => {
                inside += 1;
                score += inside;
            }
            '}' if !garbage => inside -= 1,
            '<' => garbage = true,
            '>' => garbage = false,
            '!' => skip_ch = true,
            _ => continue,
        }
    }
    println!("The answer to part 1 is {}", score);
    println!("The answer to part 2 is {}", garbage_chars);
    Ok(())
}
