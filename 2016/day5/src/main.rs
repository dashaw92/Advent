use md5::compute;

fn main() {
    let input = std::env::args()
        .nth(1)
        .expect("Expected puzzle input as args[0]");

    println!("Part 1: {}", solve_p1(&input));
    println!("\nOk, prepare the popcorn... p2 will take a long time");
    println!("Part 2: {}", solve_p2(&input));
}

fn solve_p1(input: impl AsRef<str>) -> String {
    let seed = input.as_ref();

    (0..)
        .map(|i| format!("{seed}{i}"))
        .map(compute)
        .map(|d| format!("{d:x}"))
        .filter(|digest| digest.starts_with("00000"))
        .inspect(|hex| println!("p1: found hash {hex}"))
        .map(|slice| slice.chars().nth(5).unwrap())
        .take(8)
        .collect()
}

fn solve_p2(input: impl AsRef<str>) -> String {
    let mut found = 0;
    let seed = input.as_ref();
    let mut code: [u8; 8] = [b'-'; 8];

    for i in 0.. {
        if found == 8 {
            break;
        }

        let digest = compute(format!("{seed}{i}"));
        let hex = format!("{digest:x}");

        if hex.starts_with("00000") {
            let pos = hex.chars().nth(5).and_then(|c| c.to_digit(10));
            //invalid pos, ignore this hash as per spec
            //pos already found, ignore this hash
            let pos = match pos {
                Some(p) => {
                    if p > 7 || code[p as usize] != b'-' {
                        continue;
                    }
                    p as usize
                }
                None => continue,
            };

            let val = hex.chars().nth(6).unwrap() as u8;
            code[pos] = val;
            found += 1;
            println!("p2::[{i}] Found {hex}! Remaining: {}", code.len() - found);
        }
    }

    String::from_utf8(code.to_vec()).unwrap()
}
