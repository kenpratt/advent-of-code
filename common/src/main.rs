use std::env;
use std::time::{Duration, Instant};

use advent_of_code::days;

use itertools::Itertools;

fn main() {
    let timer = Instant::now();

    let args: Vec<String> = env::args().collect_vec();
    match args.len() {
        1 => run_all(&timer),
        2 => match args.get(1).unwrap().as_str() {
            "all" => run_all(&timer),
            year => run_year(str_to_usize(year), &timer),
        },
        3 => {
            let year = str_to_usize(args.get(1).unwrap().as_str());
            let day = str_to_usize(args.get(2).unwrap().as_str());
            run_day(year, day, &timer)
        }
        _ => panic!(
            "Unexpected extra args: {:?} - maybe try moving the flags around?",
            &args
        ),
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
    println!("all took: {}ms\n", total.as_millis());
    total
}

fn str_to_usize(s: &str) -> usize {
    s.parse::<usize>().unwrap()
}
