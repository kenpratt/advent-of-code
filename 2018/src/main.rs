use std::env;
use std::time::{Duration, Instant};

use advent_of_code_2018::*;
use itertools::Itertools;

fn main() {
    let timer = Instant::now();

    let args: Vec<String> = env::args().collect_vec();
    if args.len() > 2 {
        panic!(
            "Unexpected extra args: {:?} - maybe try moving the flags around?",
            &args
        );
    }

    match args.get(1).map(|s| s.as_str()) {
        None | Some("all") => {
            run_all(&timer);
        }
        Some(day) => {
            run_day(day.parse::<u8>().unwrap(), &timer);
        }
    };
}

fn run_day(day: u8, timer: &Instant) -> Option<Duration> {
    let run = match day {
        0 => template::run,
        1 => day_01_chronal_calibration::run,
        2 => day_02_inventory_management_system::run,
        3 => day_03_no_matter_how_you_slice_it::run,
        4 => day_04_repose_record::run,
        5 => day_05_alchemical_reduction::run,
        6 => day_06_chronal_coordinates::run,
        7 => day_07_the_sum_of_its_parts::run,
        8 => day_08_memory_maneuver::run,
        9 => day_09_marble_mania::run,
        10 => day_10_the_stars_align::run,
        11 => day_11_chronal_charge::run,
        12 => day_12_subterranean_sustainability::run,
        13 => day_13_mine_cart_madness::run,
        _ => return None,
    };

    let start = timer.elapsed();
    run();
    let duration = timer.elapsed() - start;
    println!("day {} took {}ms\n", day, duration.as_millis());

    Some(duration)
}

fn run_all(timer: &Instant) {
    let total = (1..=25)
        .flat_map(|day| run_day(day, timer))
        .map(|d| d.as_millis())
        .sum::<u128>();
    println!("total: {}ms", total);
}
