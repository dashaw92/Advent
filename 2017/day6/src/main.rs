const INPUT: &'static str = "10	3	15	10	5	15	5	15	9	2	5	8	5	2	3	6";

fn main() {
    let mut steps = 1;
    let mut solved = false;
    let mut steps_two = 0;
    let mut states: Vec<Vec<i32>> = vec![];
    let mut current: Vec<i32> = INPUT
        .split("\t")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    states.push(current.clone());
    let mut part2 = vec![];
    loop {
        let mut max = *current.iter().max().unwrap(); //Thank you @jad340
        let mut index = current.iter().position(|x| *x == max).unwrap();
        current[index] = 0;
        while max > 0 {
            index += 1;
            index %= current.len();
            current[index] += 1;
            max -= 1;
        }
        if solved {
            if current == part2 {
                steps_two += 1;
                break;
            }
            steps_two += 1;
        } else {
            for state in &states {
                if current == *state {
                    solved = true;
                    part2 = current.clone();
                    steps -= 1;
                    break;
                }
            }
            steps += 1;
        }
        states.push(current.clone());
    }
    println!("The answer to part 1 is {}", steps);
    println!("The answer to part 2 is {}", steps_two);
}