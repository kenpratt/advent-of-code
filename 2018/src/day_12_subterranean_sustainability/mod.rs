use crate::file::*;

use lazy_static::lazy_static;
use regex::Regex;

pub fn run() {
    let input = parse(&read_input_file!());
    println!("part 1 result: {:?}", part1(&input));
    println!("part 2 result: {:?}", part2(&input));
}

type Notes = [bool; 32];

#[derive(Debug)]
struct Input {
    initial: (u128, u8),
    notes: Notes,
}

impl Input {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();

        let initial = Self::parse_initial_state(lines.next().unwrap());
        assert_eq!(lines.next().unwrap(), "");

        let raw_notes: Vec<(u128, u128)> = lines.map(|line| Self::parse_note(line)).collect();

        let mut notes = [false; 32];
        for (from, to) in raw_notes.into_iter() {
            if to == 1 {
                notes[from as usize] = true;
            }
        }

        Self { initial, notes }
    }

    fn parse_initial_state(input: &str) -> (u128, u8) {
        lazy_static! {
            static ref INPUT_RE: Regex = Regex::new(r"\Ainitial state: ([#\.]+)\z").unwrap();
        }

        let caps = INPUT_RE.captures(input).unwrap();
        let s = caps.get(1).unwrap().as_str();
        let state = Self::parse_num(s);
        let width = s.len() as u8;
        (state, width)
    }

    fn parse_note(input: &str) -> (u128, u128) {
        lazy_static! {
            static ref INPUT_RE: Regex = Regex::new(r"\A([#\.]+) => ([#\.])\z").unwrap();
        }

        let caps = INPUT_RE.captures(input).unwrap();
        let prev = Self::parse_num(caps.get(1).unwrap().as_str());
        let next = Self::parse_num(caps.get(2).unwrap().as_str());
        (prev, next)
    }

    fn parse_num(input: &str) -> u128 {
        let mut out = 0;
        for c in input.chars() {
            out <<= 1;
            match c {
                '#' => out |= 1,
                '.' => (),
                _ => panic!("Unexpected char: {:?}", c),
            };
        }
        out
    }
}

const U128_LEFTMOST_BIT: u128 = 170141183460469231731687303715884105728;
const PLANT_MASK: u128 = 31;

#[derive(Debug)]
struct Simulation {
    state: u128,
    width: u8,
    start_index: isize,
}

impl Simulation {
    fn initial_state(input: &Input) -> Self {
        let (state, width) = input.initial;
        Self {
            state,
            width,
            start_index: 0,
        }
    }

    fn run(input: &Input, num_generations: usize) -> Self {
        (0..num_generations).fold(Self::initial_state(input), |state, _| {
            Self::tick(&state, &input.notes)
        })
    }

    fn tick(prev: &Self, notes: &Notes) -> Self {
        // we'll want to visit numbers two on either side of the prev generation
        // #..#.#
        //      ^^^^^
        //        m
        // #..#.#
        //     ^^^^^
        //       m
        // #..#.#
        //    ^^^^^
        //      m
        // so start with prev generation shifted 4 left so it aligns with the mask
        let mut prev_state = prev.state << 4;
        let mut new_state: u128 = 0;

        let mut new_width = prev.width + 4;
        let mut new_start_index = prev.start_index - 2;

        // build a new number with two new values on either side
        // for now, this number will be on the left side of the u128 bits
        for _ in 0..new_width {
            let val = prev_state & PLANT_MASK;
            let i = val as usize;

            // bump new state to the right
            new_state >>= 1;

            if notes[i] {
                new_state |= U128_LEFTMOST_BIT;
            }

            // bump prev state to the right
            prev_state >>= 1;
        }

        // trim left side of any extra zeroes
        while (new_state & U128_LEFTMOST_BIT) == 0 {
            new_state <<= 1;
            new_width -= 1;
            new_start_index += 1;
        }

        // shift to right side of int
        new_state >>= 128 - new_width;

        // trim right side of any extra zeroes
        while (new_state & 1) == 0 {
            new_state >>= 1;
            new_width -= 1;
        }

        Self {
            state: new_state,
            width: new_width,
            start_index: new_start_index,
        }
    }

    fn sum_plant_indices(&self) -> isize {
        let mut sum = 0;
        let mut state = self.state;
        let mut index = self.start_index + self.width as isize - 1;
        for _ in 0..self.width {
            if (state & 1) == 1 {
                sum += index;
            }
            state >>= 1;
            index -= 1;
        }
        sum
    }
}

fn parse(input: &str) -> Input {
    Input::parse(input)
}

fn part1(input: &Input) -> isize {
    let sim = Simulation::run(input, 20);
    sim.sum_plant_indices()
}

fn part2(input: &Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let result = part1(&parse(&read_example_file!()));
        assert_eq!(result, 325);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&parse(&read_input_file!()));
        assert_eq!(result, 1816);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(&parse(&read_example_file!()));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&parse(&read_input_file!()));
        assert_eq!(result, 0);
    }
}
