const INPUT: i32 = 368078;

fn main() {
    let mut iter = 515;
    while INPUT - (iter * iter) > iter {
        iter += 2;
    }
    let rcorner = iter * iter;
    let mid = rcorner - (iter / 2);
    let mut steps = 0;
    if INPUT > mid {
        steps += INPUT - mid;
    } else {
        steps += mid - INPUT;
    }
    steps += iter / 2;
    println!("The answer to part 1 is {}", steps);
}
