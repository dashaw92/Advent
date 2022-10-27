use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

mod clutter;

use clutter::*;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;
    println!("Part 1: {}", solve_p1(&input));
    Ok(())
}

fn solve_p1(input: impl AsRef<str>) -> i32 {
    let events = parse_events(input.as_ref());

    let mut sleep_time: HashMap<i32, i32> = HashMap::new();

    //(Guard ID, Minute) => Times slept on THIS minute
    let mut minute_freq: HashMap<(i32, i32), i32> = HashMap::new();
    let mut current = 0;
    let mut start_sleep_min = 0;
    for event in &events {
        match event.what {
            Action::StartShift(id) => current = id,
            Action::Sleep => start_sleep_min = event.time.m,
            Action::Wake => {
                let delta = event.time.m - start_sleep_min;
                *sleep_time.entry(current).or_insert(0) += delta;

                (start_sleep_min..event.time.m).for_each(|minute| {
                    *minute_freq.entry((current, minute)).or_insert(0) += 1;
                })
            }
        }
    }

    let (id, _) = sleep_time.iter().max_by_key(|(_, &time)| time).unwrap();
    let ((_, minute), _) = minute_freq
        .iter()
        .filter(|((gid, _), _)| gid == id)
        .max_by_key(|((_, _), &freq)| freq)
        .unwrap();

    id * minute
}

// fn solve_p2(input: impl AsRef<str>) -> usize {
//     0
// }

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    #[test]
    fn provided_p1() {
        assert_eq!(240, solve_p1(PROVIDED));
    }

    #[test]
    fn provided_p2() {
        assert_eq!(0, solve_p2(PROVIDED));
    }
}
