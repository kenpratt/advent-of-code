use crate::{interface::AoC, math::*};

use lazy_static::lazy_static;
use regex::Regex;

pub struct Day;
impl AoC<Input, isize, isize> for Day {
    const FILE: &'static str = file!();

    fn parse(input: String) -> Input {
        Input::parse(&input)
    }

    fn part1(input: &Input) -> isize {
        let sim = Simulation::run(input, 20);
        sim.sum_plant_indices()
    }

    fn part2(input: &Input) -> isize {
        let sim = Simulation::run(input, 50000000000);
        sim.sum_plant_indices()
    }
}

type Notes = [bool; 32];

#[derive(Debug)]
pub struct Input {
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

#[derive(Debug)]
struct Simulation {
    state: U256,
    width: u8,
    start_index: isize,
}

impl Simulation {
    fn initial_state(input: &Input) -> Self {
        let (state, width) = input.initial;
        Self {
            state: U256::from_u128(state),
            start_index: 0,
            width,
        }
    }

    fn run(input: &Input, num_generations: usize) -> Self {
        let mut sim = Self::initial_state(input);
        let plant_mask = U256::from_u128(31);

        for generation in 1..=num_generations {
            let next_sim = Self::tick(&sim, &input.notes, &plant_mask);

            if sim.state == next_sim.state {
                // we found a repeating pattern!
                assert_eq!(sim.width, next_sim.width);

                let index_diff = next_sim.start_index - sim.start_index;
                let remaining = (num_generations - generation) as isize;
                let start_index = next_sim.start_index + (remaining * index_diff);

                return Self {
                    state: sim.state,
                    width: sim.width,
                    start_index,
                };
            } else {
                sim = next_sim;
            }
        }

        sim
    }

    fn tick(prev: &Self, notes: &Notes, plant_mask: &U256) -> Self {
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
        let mut new_state: U256 = U256::new();

        let mut new_width = prev.width + 4;
        let mut new_start_index = prev.start_index - 2;

        // build a new number with two new values on either side
        // for now, this number will be on the left side of the u128 bits
        // let seen_one = false;
        for _ in 0..new_width {
            let val = prev_state & *plant_mask;
            let i = val.as_usize();

            // bump new state to the right
            new_state >>= 1;

            if notes[i] {
                new_state |= U256::LEFTMOST_BIT;
            }

            // bump prev state to the right
            prev_state >>= 1;
        }

        // trim left side of any extra zeroes
        while (new_state & U256::LEFTMOST_BIT) == 0 {
            new_state <<= 1;
            new_width -= 1;
            new_start_index += 1;
        }

        // shift to right side of int
        new_state >>= 256 - new_width as usize;

        // trim right side of any extra zeroes
        while (new_state & U256::ONE) == 0 {
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
            if (state & U256::ONE) == 1 {
                sum += index;
            }
            state >>= 1;
            index -= 1;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let result = Day::part1(&Day::parse_example_file());
        assert_eq!(result, 325);
    }

    #[test]
    fn test_part1_solution() {
        let result = Day::part1(&Day::parse_input_file());
        assert_eq!(result, 1816);
    }

    #[test]
    fn test_part2_example() {
        let result = Day::part2(&Day::parse_example_file());
        assert_eq!(result, 999999999374);
    }

    #[test]
    fn test_part2_solution() {
        let result = Day::part2(&Day::parse_input_file());
        assert_eq!(result, 399999999957);
    }
}
