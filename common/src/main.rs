use std::env;
use std::time::{Duration, Instant};

use advent_of_code::days;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    lazy_static! {
        static ref YEAR_RE: Regex = Regex::new(r"\A(\d{4})\z").unwrap();
        static ref DAY_RE: Regex = Regex::new(r"\A(\d{1,2})\z").unwrap();
    }

    let timer = Instant::now();

    let args: Vec<String> = env::args().into_iter().skip(1).collect();

    match args.get(0).map(|s| s.as_str()) {
        // run all
        None | Some("all") => {
            assert!(args.len() <= 1);
            run_all(&timer)
        }

        // run year or day
        Some(year_s) if YEAR_RE.is_match(year_s) => {
            let year = str_to_usize(year_s);
            match args.get(1) {
                None => run_year(year, &timer),
                Some(day_s) if DAY_RE.is_match(day_s) => {
                    let day = str_to_usize(day_s);
                    run_day(year, day, &timer)
                }
                Some(_) => panic!("Unexpected args: {:?}", args),
            }
        }

        // benchmark a day
        Some("bench") => {
            let year_s = &args[1];
            let day_s = &args[2];
            let times_s = &args[3];
            assert!(YEAR_RE.is_match(year_s));
            assert!(DAY_RE.is_match(day_s));
            let year = str_to_usize(year_s);
            let day = str_to_usize(day_s);
            let times = str_to_usize(times_s);
            benchmark_day(year, day, times, &timer)
        }

        Some(_) => panic!("Unexpected args: {:?}", args),
    };
}

fn run_day(year: usize, day: usize, timer: &Instant) -> Duration {
    let run = days::run_fn(year, day).unwrap();
    let start = timer.elapsed();
    run();
    let duration = timer.elapsed() - start;
    println!("{} day {} took {}ms\n", year, day, duration.as_millis());
    duration
}

fn run_year(year: usize, timer: &Instant) -> Duration {
    let total: Duration = days::days_for_year(year)
        .iter()
        .map(|d| run_day(year, *d, timer))
        .sum();
    println!("{} total: {}ms\n", year, total.as_millis());
    total
}

fn run_all(timer: &Instant) -> Duration {
    let total: Duration = days::YEARS.iter().map(|year| run_year(*year, timer)).sum();
    println!("all years total: {}ms\n", total.as_millis());
    total
}

fn benchmark_day(year: usize, day: usize, times: usize, timer: &Instant) -> Duration {
    let mut durations: Vec<Duration> = (0..times).map(|_| run_day(year, day, timer)).collect();
    durations.sort();
    let median = durations[durations.len() / 2];
    println!(
        "{} day {}, with {} runs:\n  median {}ms\n  min {}ms\n  max {}ms\n",
        year,
        day,
        times,
        median.as_millis(),
        durations.first().unwrap().as_millis(),
        durations.last().unwrap().as_millis()
    );
    median
}

fn str_to_usize(s: &str) -> usize {
    s.parse::<usize>().unwrap()
}
