use md5::compute;

fn main() {
    let input = std::env::args()
        .nth(1)
        .expect("Expected puzzle input as args[0]");

    println!("Part 1: {}", solve_p1(&input));
}

fn solve_p1(input: impl AsRef<str>) -> String {
    let seed = input.as_ref();

    (0..)
        .map(|i| format!("{seed}{i}"))
        .map(compute)
        .map(|d| format!("{d:x}"))
        .filter(|digest| digest.starts_with("00000"))
        .map(|slice| slice.chars().nth(5).unwrap())
        .take(8)
        .collect()
}
