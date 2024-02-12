// generated by build.rs
use crate::interface::*;
use crate::template;
use crate::y2015;
use crate::y2018;

pub const YEARS: [usize; 2] = [
    2015,
    2018,
];

pub const DAYS_2015: [usize; 7] = [
    1,
    2,
    3,
    4,
    5,
    6,
    7,
];

pub const DAYS_2018: [usize; 13] = [
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
];

pub fn days_for_year(year: usize) -> &'static [usize] {
    match year {
        2015 => &DAYS_2015,
        2018 => &DAYS_2018,
        _ => panic!("Unknown year: {}", year),
    }
}

pub fn run_fn(year: usize, day: usize) -> Option<fn(bool)> {
    match (year, day) {
        (_, 0) => Some(template::Day::run),
        (2015, 1) => Some(y2015::day_01_not_quite_lisp::Day::run),
        (2015, 2) => Some(y2015::day_02_i_was_told_there_would_be_no_math::Day::run),
        (2015, 3) => Some(y2015::day_03_perfectly_spherical_houses_in_a_vacuum::Day::run),
        (2015, 4) => Some(y2015::day_04_the_ideal_stocking_stuffer::Day::run),
        (2015, 5) => Some(y2015::day_05_doesnt_he_have_intern_elves_for_this::Day::run),
        (2015, 6) => Some(y2015::day_06_probably_a_fire_hazard::Day::run),
        (2015, 7) => Some(y2015::day_07_some_assembly_required::Day::run),
        (2018, 1) => Some(y2018::day_01_chronal_calibration::Day::run),
        (2018, 2) => Some(y2018::day_02_inventory_management_system::Day::run),
        (2018, 3) => Some(y2018::day_03_no_matter_how_you_slice_it::Day::run),
        (2018, 4) => Some(y2018::day_04_repose_record::Day::run),
        (2018, 5) => Some(y2018::day_05_alchemical_reduction::Day::run),
        (2018, 6) => Some(y2018::day_06_chronal_coordinates::Day::run),
        (2018, 7) => Some(y2018::day_07_the_sum_of_its_parts::Day::run),
        (2018, 8) => Some(y2018::day_08_memory_maneuver::Day::run),
        (2018, 9) => Some(y2018::day_09_marble_mania::Day::run),
        (2018, 10) => Some(y2018::day_10_the_stars_align::Day::run),
        (2018, 11) => Some(y2018::day_11_chronal_charge::Day::run),
        (2018, 12) => Some(y2018::day_12_subterranean_sustainability::Day::run),
        (2018, 13) => Some(y2018::day_13_mine_cart_madness::Day::run),
        _ => return None,
    }
}
