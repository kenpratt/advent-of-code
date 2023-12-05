use std::{cmp, collections::HashMap, fs, str::Lines};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<(usize, usize)>,
    maps: HashMap<Resource, ConversionMap>,
}

impl Almanac {
    fn parse(input: &str, parse_with_seed_ranges: bool) -> Self {
        let mut parts = input.split("\n\n");

        let seeds_str: &str = parts.next().unwrap();
        let mut seeds = Self::parse_seeds(seeds_str, parse_with_seed_ranges);
        seeds.sort_by_cached_key(|(index, _len)| *index);

        let maps_list: Vec<ConversionMap> = parts.map(|p| ConversionMap::parse(p)).collect();
        let maps = maps_list
            .into_iter()
            .map(|m| (m.source_category.clone(), m))
            .collect();

        Self { seeds, maps }
    }

    fn parse_seeds(input: &str, parse_with_seed_ranges: bool) -> Vec<(usize, usize)> {
        lazy_static! {
            static ref SEEDS_RE: Regex = Regex::new(r"\Aseeds: ([\d\s]+)\z").unwrap();
        }

        let caps = SEEDS_RE.captures(input).unwrap();
        let nums = parse_numbers(caps.get(1).unwrap().as_str());

        if parse_with_seed_ranges {
            nums.chunks(2).map(|pair| (pair[0], pair[1])).collect()
        } else {
            nums.into_iter().map(|n| (n, 1)).collect()
        }
    }

    fn minimum_location(&self) -> usize {
        let resulting_ranges = self.convert_seeds(&Resource::Location);
        resulting_ranges
            .into_iter()
            .map(|(index, _length)| index)
            .min()
            .unwrap()
    }

    fn convert_seeds(&self, final_category: &Resource) -> Vec<(usize, usize)> {
        let mut source_ranges = self.seeds.clone();
        let mut source_category = Resource::Seed;

        while &source_category != final_category {
            let mut destination_ranges = vec![];
            let mut destination_category = source_category;

            for source_range in source_ranges {
                let (mut this_destination_ranges, this_destination_category) =
                    self.convert_resource(source_range, &source_category);
                destination_ranges.append(&mut this_destination_ranges);
                destination_category = this_destination_category;
            }

            source_ranges = destination_ranges;
            source_category = destination_category;
        }

        source_ranges
    }

    fn convert_resource(
        &self,
        source_range: (usize, usize),
        source_category: &Resource,
    ) -> (Vec<(usize, usize)>, Resource) {
        let map = self.maps.get(source_category).unwrap();
        let dest_ranges = map.convert(source_range);
        (dest_ranges, map.destination_category)
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
        let conversions = Conversion::parse_list(lines);

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

    fn convert(&self, source_range: (usize, usize)) -> Vec<(usize, usize)> {
        let (want_index, want_length) = source_range;

        // find index of first range
        let starting_index = self
            .conversions
            .iter()
            .position(|c| c.contains_source_index(want_index))
            .unwrap();

        let mut out = vec![];

        let mut remaining_index = want_index;
        let mut remaining_length = want_length;

        for conversion in &self.conversions[starting_index..] {
            let offset = remaining_index - conversion.source_index;

            let dest_index = conversion.destination_index + offset;
            let available_length = conversion.length - offset;
            let dest_length = cmp::min(remaining_length, available_length);

            out.push((dest_index, dest_length));

            remaining_index = conversion.source_index_after_end();
            remaining_length -= dest_length;

            if remaining_length == 0 {
                break;
            }
        }

        out
    }
}

#[derive(Debug)]
struct Conversion {
    source_index: usize,
    destination_index: usize,
    length: usize,
}

impl Conversion {
    fn parse_list(lines: Lines<'_>) -> Vec<Self> {
        let mut tmp: Vec<Conversion> = lines.map(|l| Self::parse(l)).collect();
        tmp.sort_by_cached_key(|c| c.source_index);

        let mut res: Vec<Conversion> = vec![];

        // fill initial gap
        let first = tmp.first().unwrap();
        if first.source_index > 0 {
            let gap = Conversion {
                destination_index: 0,
                source_index: 0,
                length: first.source_index,
            };
            res.push(gap);
        }

        // fill other gaps
        for curr in tmp {
            match res.last() {
                Some(last) => {
                    let gap_index = last.source_index_after_end();
                    if curr.source_index > gap_index {
                        let gap_length = curr.source_index - gap_index;
                        let gap = Conversion {
                            destination_index: gap_index,
                            source_index: gap_index,
                            length: gap_length,
                        };
                        res.push(gap);
                    } else if curr.source_index < gap_index {
                        panic!(
                            "Unexpected overlap: {:?}, {:?}, {:?}",
                            curr, last, gap_index
                        );
                    }
                }
                None => (),
            }
            res.push(curr);
        }

        // add buffer at end
        let last: &Conversion = res.last().unwrap();
        let end_index = last.source_index_after_end();
        let end_length = usize::MAX - end_index - 1;
        let gap = Conversion {
            source_index: end_index,
            destination_index: end_index,
            length: end_length,
        };
        res.push(gap);

        // verify no gaps
        let mut expected_index = 0;
        for elem in &res {
            assert_eq!(elem.source_index, expected_index);
            expected_index = elem.source_index + elem.length;
        }

        res
    }

    fn parse(input: &str) -> Self {
        let nums = parse_numbers(input);
        assert_eq!(nums.len(), 3);

        Conversion {
            destination_index: nums[0],
            source_index: nums[1],
            length: nums[2],
        }
    }

    fn source_index_after_end(&self) -> usize {
        self.source_index + self.length
    }

    fn contains_source_index(&self, index: usize) -> bool {
        index >= self.source_index && index < self.source_index_after_end()
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
    let almanac = Almanac::parse(input, false);
    almanac.minimum_location()
}

fn part2(input: &str) -> usize {
    let almanac = Almanac::parse(input, true);
    almanac.minimum_location()
}

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

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 46);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 26829166);
    }
}
