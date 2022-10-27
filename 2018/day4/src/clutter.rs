use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub time: Time,
    pub what: Action,
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

impl FromStr for Event {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (time, rest) = s.split_once("] ").unwrap();
        let time = time.parse::<Time>()?;

        if rest.contains("Guard #") {
            let id = rest
                .split_once('#')
                .unwrap()
                .1
                .split_once(' ')
                .unwrap()
                .0
                .trim()
                .parse::<i32>()
                .unwrap();

            Ok(Event {
                time,
                what: Action::StartShift(id),
            })
        } else if rest.contains("falls asleep") {
            Ok(Event {
                time,
                what: Action::Sleep,
            })
        } else if rest.contains("wakes up") {
            Ok(Event {
                time,
                what: Action::Wake,
            })
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Action {
    StartShift(i32),
    Sleep,
    Wake,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Time {
    pub y: i32,
    pub mo: i32,
    pub d: i32,
    pub h: i32,
    pub m: i32,
}

impl std::fmt::Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:04}-{:02}-{:02} {:02}:{:02}]",
            self.y, self.mo, self.d, self.h, self.m
        )
    }
}

impl FromStr for Time {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chopped = s.strip_prefix('[').map(|s| s.trim()).unwrap();

        let (date, time) = chopped.split_once(' ').unwrap();
        let date: Vec<i32> = date
            .split('-')
            .filter_map(|part| part.parse::<i32>().ok())
            .collect();

        let time: Vec<i32> = time
            .split(':')
            .filter_map(|part| part.parse::<i32>().ok())
            .collect();

        Ok(Time {
            y: date[0],
            mo: date[1],
            d: date[2],
            h: time[0],
            m: time[1],
        })
    }
}

pub fn parse_events(input: &str) -> Vec<Event> {
    let mut events: Vec<Event> = input.lines().filter_map(|line| line.parse().ok()).collect();
    events.sort();

    events
}
