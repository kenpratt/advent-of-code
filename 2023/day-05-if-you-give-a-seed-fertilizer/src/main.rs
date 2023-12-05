use std::{collections::HashMap, fs};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: HashMap<Resource, ConversionMap>,
}

impl Almanac {
    fn parse(input: &str) -> Self {
        let mut parts = input.split("\n\n");

        let seeds_str: &str = parts.next().unwrap();
        let seeds = Self::parse_seeds(seeds_str);

        let maps_list: Vec<ConversionMap> = parts.map(|p| ConversionMap::parse(p)).collect();
        let maps = maps_list
            .into_iter()
            .map(|m| (m.source_category.clone(), m))
            .collect();

        Self { seeds, maps }
    }

    fn parse_seeds(input: &str) -> Vec<usize> {
        lazy_static! {
            static ref SEEDS_RE: Regex = Regex::new(r"\Aseeds: ([\d\s]+)\z").unwrap();
        }

        let caps = SEEDS_RE.captures(input).unwrap();
        parse_numbers(caps.get(1).unwrap().as_str())
    }

    fn convert_seeds(&self, final_category: &Resource) -> Vec<usize> {
        self.seeds
            .iter()
            .map(|s| self.convert_seed(s, final_category))
            .collect()
    }

    fn convert_seed(&self, seed_id: &usize, final_category: &Resource) -> usize {
        let mut source_id = *seed_id;
        let mut source_category = Resource::Seed;

        while &source_category != final_category {
            let (next_id, next_category) = self.convert_resource(&source_id, &source_category);
            source_id = next_id;
            source_category = next_category;
        }

        source_id
    }

    fn convert_resource(&self, source_id: &usize, source_category: &Resource) -> (usize, Resource) {
        let map = self.maps.get(source_category).unwrap();
        let dest_id = map.convert(source_id);
        (dest_id, map.destination_category)
    }
}

#[derive(Debug)]
struct ConversionMap {
    source_category: Resource,
    destination_category: Resource,
    conversions: Vec<Conversion>,
}

impl ConversionMap {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();

        let (source_category, destination_category) = Self::parse_header(lines.next().unwrap());

        let conversions = lines.map(|l| Conversion::parse(l)).collect();

        Self {
            source_category,
            destination_category,
            conversions,
        }
    }

    fn parse_header(input: &str) -> (Resource, Resource) {
        lazy_static! {
            static ref HEADER_RE: Regex = Regex::new(r"\A([a-z]+)\-to\-([a-z]+) map:\z").unwrap();
        }

        let caps = HEADER_RE.captures(input).unwrap();
        let source = Resource::parse(caps.get(1).unwrap().as_str());
        let dest = Resource::parse(caps.get(2).unwrap().as_str());
        (source, dest)
    }

    fn convert(&self, source_id: &usize) -> usize {
        for conversion in &self.conversions {
            match conversion.convert(source_id) {
                Some(destination_id) => return destination_id,
                None => (), // continue
            }
        }
        *source_id // fallback is the same ID
    }
}

#[derive(Debug)]
struct Conversion {
    destination_index: usize,
    source_index: usize,
    length: usize,
}

impl Conversion {
    fn parse(input: &str) -> Self {
        let nums = parse_numbers(input);
        assert_eq!(nums.len(), 3);

        Conversion {
            destination_index: nums[0],
            source_index: nums[1],
            length: nums[2],
        }
    }

    fn convert(&self, source_id: &usize) -> Option<usize> {
        if *source_id >= self.source_index {
            let offset = source_id - self.source_index;
            if offset < self.length {
                let destination_id = self.destination_index + offset;
                Some(destination_id)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Resource {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl Resource {
    fn parse(input: &str) -> Self {
        use Resource::*;

        match input {
            "seed" => Seed,
            "soil" => Soil,
            "fertilizer" => Fertilizer,
            "water" => Water,
            "light" => Light,
            "temperature" => Temperature,
            "humidity" => Humidity,
            "location" => Location,
            _ => panic!("Unknown resource type: {}", input),
        }
    }
}

fn parse_numbers(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn part1(input: &str) -> usize {
    let almanac = Almanac::parse(input);

    let results = almanac.convert_seeds(&Resource::Location);
    results.into_iter().min().unwrap()
}

// fn part2(input: &str) -> usize {
//     let almanacs = Data::parse(input);
//     dbg!(&almanacs);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 278755257);
    }

    // #[test]
    // fn test_part2_example() {
    //     let result = part2(EXAMPLE);
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}
