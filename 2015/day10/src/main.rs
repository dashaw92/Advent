fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let input: i64 = args[0].parse().unwrap();
    let iters: i32 = args[1].parse().unwrap_or(40);

    println!("Part 1: {}", solve_p1(input, iters));
}

//take starting digit, apply the algorithm iters times
//return length of output
fn solve_p1(input: i64, iters: i32) -> usize {
    let mut work = input.to_string();

    for _ in 0..iters {
        let mut buf = String::new();
        let chars: Vec<char> = work.chars().collect();

        let mut idx = 0;
        while idx < chars.len() {
            let current = chars[idx];
            let mut run = 1;
            if idx + 1 < chars.len() {
                let mut next = idx + 1;
                while next < chars.len() && chars[next] == current {
                    run += 1;
                    next += 1;
                }
            }

            buf.push_str(&format!("{run}{current}"));
            idx += run;
        }

        work = buf;
    }

    work.len()
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: i64 = 1;

    #[test]
    fn provided_p1() {
        assert_eq!(6, solve_p1(PROVIDED, 5));
    }
}
