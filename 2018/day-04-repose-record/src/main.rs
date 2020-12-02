extern crate chrono;
extern crate regex;

use chrono::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    // read input & split into lines
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let lines = contents.lines();

    // parse input
    let mut log: Vec<Entry> = lines.map(|l| Entry::parse(l)).collect();

    // sort by time
    log.sort_unstable_by_key(|e| e.time);

    //println!("log: {:?}", log);

    let guards = build_guard_map(&log);
    //println!("guards: {:?}", guards);

    for (id, data) in guards.iter() {
        println!("{:?}: {:?}, {:?}", id, data.total_nap_duration(), data.sleepiest_minute());
    }


    let part1 = part1(&guards);
    println!("part 1: {:?}", part1);

    let part2 = part2(&guards);
    println!("part 2: {:?}", part2);
}

fn part1(guards: &HashMap<u16, GuardMetadata>) -> u32 {
    let target = guards.iter().max_by_key(|(_id, data)| data.total_nap_duration()).unwrap();

    let id = target.0;
    let sleepiest_minute = target.1.sleepiest_minute();
    let minute = sleepiest_minute.0;
    let result = (*id as u32) * (minute as u32);

    println!("guard {}, minute: {:?}, result: {}", id, sleepiest_minute, result);

    return result;
}

fn part2(guards: &HashMap<u16, GuardMetadata>) -> u32 {
    let target = guards.iter().max_by_key(|(_id, data)| data.sleepiest_minute().1).unwrap();

    let id = target.0;
    let sleepiest_minute = target.1.sleepiest_minute();
    let minute = sleepiest_minute.0;
    let result = (*id as u32) * (minute as u32);

    println!("guard {}, minute: {:?}, result: {}", id, sleepiest_minute, result);

    return result;
}

fn build_guard_map(entries: &[Entry]) -> HashMap<u16, GuardMetadata> {
    let mut output = HashMap::new();
    let mut curr_shift_id = None;
    let mut curr_nap_start: Option<DateTime<Utc>> = None;

    for entry in entries {
        match entry.status {
            Status::BeginsShift(id) => {
                assert!(curr_nap_start.is_none(), "curr_nap_start is Some during WakesUp");

                if !output.contains_key(&id) {
                    output.insert(id, vec![]);
                }

                let shift = Shift {
                    start_time: entry.time,
                    naps: vec![],
                };
                output.get_mut(&id).unwrap().push(shift);

                curr_shift_id = Some(id);
                curr_nap_start = None;
            },
            Status::FallsAsleep => {
                assert!(curr_shift_id.is_some(), "curr_shift_id is None during FallsAsleep");
                assert!(curr_nap_start.is_none(), "curr_nap_start is Some during WakesUp");

                curr_nap_start = Some(entry.time);
            },
            Status::WakesUp => {
                assert!(&curr_shift_id.is_some(), "curr_shift_id is None during FallsAsleep");
                assert!(curr_nap_start.is_some(), "curr_nap_start is None during WakesUp");

                let nap = Nap {
                    start_time: curr_nap_start.unwrap(),
                    end_time: entry.time,
                };

                let shift_id = curr_shift_id.unwrap();
                let shift = output.get_mut(&shift_id).unwrap().last_mut().unwrap();
                shift.naps.push(nap);

                curr_nap_start = None;
            },
        }
    }

    return output;
}

type GuardMetadata = Vec<Shift>;

trait GuardMetadataHelpers {
    fn total_nap_duration(&self) -> i64;
    fn sleepiest_minute(&self) -> (u8, u8);
}

impl GuardMetadataHelpers for GuardMetadata {
    fn total_nap_duration(&self) -> i64 {
        return self.iter().map(|shift| shift.total_nap_duration()).sum();
    }
    fn sleepiest_minute(&self) -> (u8, u8) {
        let mut v: [u8; 60] = [0; 60];

        for shift in self {
            for nap in &shift.naps {
                for minute in nap.start_time.minute()..nap.end_time.minute() {
                    v[minute as usize] += 1;
                }
            }
        }

        let with_indices: Vec<(usize, &u8)> = v.iter().enumerate().collect();
        let sleepiest = with_indices.iter().max_by_key(|(_minute, total)| total);

        let minute = sleepiest.unwrap().0 as u8;
        let nap_count = *sleepiest.unwrap().1 as u8;
        return (minute, nap_count);
    }
}

#[derive(Debug)]
struct Shift {
    start_time: DateTime<Utc>,
    //end_time: DateTime<Utc>,
    naps: Vec<Nap>,
}

impl Shift {
    fn total_nap_duration(&self) -> i64 {
        return self.naps.iter().map(|nap| nap.duration()).sum();
    }
}

#[derive(Debug)]
struct Nap {
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}

impl Nap {
    fn duration(&self) -> i64 {
        return (self.end_time - self.start_time).num_minutes();
    }
}

#[derive(Debug)]
struct Entry {
    time: DateTime<Utc>,
    status: Status,
}

#[derive(Debug)]
enum Status {
    BeginsShift(u16),
    FallsAsleep,
    WakesUp,
}

impl Entry {
    fn parse(line: &str) -> Entry {
        let re = Regex::new(r"\[(.+)\] (.+)").expect("failed to match regexp");
        let m = re.captures(line).expect("failed to unpack re captures");

        let time_str = m.get(1).map_or("", |m| m.as_str());
        let status_str = m.get(2).map_or("", |m| m.as_str());

        let time = Entry::parse_time(time_str).expect("failed to parse time");
        let status = Entry::parse_status(status_str).expect("failed to parse status");

        return Entry {
            time: time,
            status: status,
        }
    }

    fn parse_time(time: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
        return Utc.datetime_from_str(time, "%Y-%m-%d %H:%M");
    }

    fn parse_status(status: &str) -> Result<Status, &str> {
        let begins_shift = Regex::new(r"Guard #(\d+) begins shift").expect("failed to construct regexp");

        if status == "falls asleep" {
            return Ok(Status::FallsAsleep);
        } else if status == "wakes up" {
            return Ok(Status::WakesUp);
        } else if begins_shift.is_match(status) {
            let m = begins_shift.captures(status).expect("failed to unpack re captures");
            let id_str = m.get(1).map_or("", |m| m.as_str());
            let id = id_str.parse::<u16>().unwrap();
            return Ok(Status::BeginsShift(id));
        } else {
            return Err("unknown status string");
        }
    }
}
