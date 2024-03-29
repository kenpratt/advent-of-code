use std::{collections::HashMap, ops::Range};

use lazy_static::lazy_static;
use regex::Regex;
use time::{macros::format_description, PrimitiveDateTime};

use crate::interface::AoC;

pub struct Day;
impl AoC<HashMap<u16, Vec<Shift>>, usize, usize> for Day {
    const FILE: &'static str = file!();

    fn parse(input: String) -> HashMap<u16, Vec<Shift>> {
        let shifts = Shift::parse_list(&input);
        let mut by_guard: HashMap<u16, Vec<Shift>> = HashMap::new();
        for shift in shifts {
            by_guard.entry(shift.guard).or_default().push(shift);
        }

        by_guard
    }

    fn part1(by_guard: &HashMap<u16, Vec<Shift>>) -> usize {
        let (sleepiest_guard, sleepiest_shifts) = by_guard
            .iter()
            .max_by_key(|(_guard, shifts)| {
                shifts
                    .iter()
                    .map(|shift| shift.total_nap_time())
                    .sum::<usize>()
            })
            .unwrap();

        let (minute, _count) = Shift::sleepiest_minute(sleepiest_shifts);
        *sleepiest_guard as usize * minute as usize
    }

    fn part2(by_guard: &HashMap<u16, Vec<Shift>>) -> usize {
        let (guard, (minute, _count)) = by_guard
            .iter()
            .map(|(guard, shifts)| (guard, Shift::sleepiest_minute(shifts)))
            .max_by_key(|(_guard, (_minute, count))| *count)
            .unwrap();
        *guard as usize * minute
    }
}

#[derive(Debug)]
struct Record {
    time: PrimitiveDateTime,
    entry: Entry,
}

impl Record {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref RECORD_RE: Regex = Regex::new(r"\A\[(.+)\] (.+)\z").unwrap();
        }

        let time_format = format_description!("[year]-[month]-[day] [hour]:[minute]");

        let caps = RECORD_RE.captures(input).unwrap();
        let time = PrimitiveDateTime::parse(caps.get(1).unwrap().as_str(), time_format).unwrap();
        let entry = Entry::parse(caps.get(2).unwrap().as_str());
        Self { time, entry }
    }
}

const SLEEP: &'static str = "falls asleep";
const WAKE: &'static str = "wakes up";

#[derive(Debug)]
enum Entry {
    Begin(u16),
    Sleep,
    Wake,
}

impl Entry {
    fn parse(input: &str) -> Self {
        use Entry::*;

        lazy_static! {
            static ref SHIFT_RE: Regex = Regex::new(r"\AGuard #(\d+) begins shift\z").unwrap();
        }

        match input {
            SLEEP => Sleep,
            WAKE => Wake,
            _ => match SHIFT_RE.captures(input) {
                Some(caps) => Begin(caps.get(1).unwrap().as_str().parse::<u16>().unwrap()),
                None => panic!("Unexpected entry format: {:?}", input),
            },
        }
    }
}

#[derive(Debug)]
pub struct Shift {
    guard: u16,
    naps: Vec<Range<u8>>,
}

impl Shift {
    fn parse_list(input: &str) -> Vec<Self> {
        let mut records = Record::parse_list(input);
        records.sort_by_cached_key(|r| r.time);

        let mut records_iter = records.iter().peekable();

        let mut shifts = vec![];
        while let Some(shift) = Self::next_shift(&mut records_iter) {
            shifts.push(shift);
        }
        shifts
    }

    fn next_shift<'a, I: Iterator<Item = &'a Record>>(
        iter: &mut std::iter::Peekable<I>,
    ) -> Option<Self> {
        match iter.next() {
            Some(Record {
                time: _,
                entry: Entry::Begin(guard),
            }) => {
                let mut naps = vec![];
                while let Some(nap) = Self::next_nap(iter) {
                    naps.push(nap);
                }

                Some(Self {
                    guard: *guard,
                    naps,
                })
            }
            Some(r) => panic!("Expected record to be the beginning of a shift: {:?}", r),
            None => None,
        }
    }

    fn next_nap<'a, I: Iterator<Item = &'a Record>>(
        iter: &mut std::iter::Peekable<I>,
    ) -> Option<Range<u8>> {
        match iter.peek() {
            Some(Record {
                time: _,
                entry: Entry::Begin(_),
            }) => None,
            Some(Record {
                time: sleep,
                entry: Entry::Sleep,
            }) => {
                iter.next();
                match iter.next() {
                    Some(Record {
                        time: wake,
                        entry: Entry::Wake,
                    }) => {
                        assert_eq!(sleep.date(), wake.date());
                        assert_eq!(sleep.hour(), wake.hour());
                        Some(sleep.minute()..wake.minute())
                    }
                    r => panic!("Expected wake, got {:?}", r),
                }
            }
            Some(Record {
                time,
                entry: Entry::Wake,
            }) => panic!("Expected sleep or new shift, got wake at: {:?}", time),
            None => None,
        }
    }

    fn total_nap_time(&self) -> usize {
        self.naps.iter().map(|r| r.len()).sum()
    }

    fn sleepiest_minute(shifts: &[Shift]) -> (usize, usize) {
        let mut mins: Vec<usize> = vec![0; 60];

        for shift in shifts {
            for nap in &shift.naps {
                for min in nap.clone() {
                    mins[min as usize] += 1;
                }
            }
        }

        mins.into_iter()
            .enumerate()
            .max_by_key(|(_, v)| *v)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let result = Day::part1(&Day::parse_example_file());
        assert_eq!(result, 240);
    }

    #[test]
    fn test_part1_solution() {
        let result = Day::part1(&Day::parse_input_file());
        assert_eq!(result, 125444);
    }

    #[test]
    fn test_part2_example() {
        let result = Day::part2(&Day::parse_example_file());
        assert_eq!(result, 4455);
    }

    #[test]
    fn test_part2_solution() {
        let result = Day::part2(&Day::parse_input_file());
        assert_eq!(result, 18325);
    }
}
