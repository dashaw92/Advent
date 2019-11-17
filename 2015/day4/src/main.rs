const INPUT: &'static str = "iwrupvqb";

fn main() {
    println!("The answer to part 1 is {}", solve(false));
    println!("Solving for part 2 may take some time to complete.");
    println!("The answer to part 2 is {}", solve(true));
}

fn solve(part2: bool) -> i32 {
    let search = if !part2 { "00000" } else { "000000" };
    let answer = (1..)
        .filter(|x| {
            let key = &format!("{}{}", INPUT, x);
            format!("{:x}", md5::compute(key)).starts_with(search)
        })
        .take(1)
        .nth(0)
        .unwrap_or(-1);

    answer
}
