use std::{
    collections::BTreeSet,
    fs,
    ops::{Range, RangeInclusive},
};

use lazy_static::lazy_static;
use regex::Regex;

const INPUT_FILE: &'static str = "input.txt";

fn main() {
    println!("part 1 result: {:?}", part1(&read_file(INPUT_FILE)));
    println!("part 2 result: {:?}", part2(&read_file(INPUT_FILE)));
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Claim {
    id: u16,
    x_range: Range<u16>,
    y_range: Range<u16>,
}

impl Claim {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref CLAIM_RE: Regex =
                Regex::new(r"\A#(\d+) @ (\d+),(\d+): (\d+)x(\d+)\z").unwrap();
        }

        let caps = CLAIM_RE.captures(input).unwrap();
        let mut nums_iter = caps
            .iter()
            .skip(1)
            .flat_map(|c| c.unwrap().as_str().parse::<u16>());

        let id = nums_iter.next().unwrap();
        let left = nums_iter.next().unwrap();
        let top = nums_iter.next().unwrap();
        let width = nums_iter.next().unwrap();
        let height = nums_iter.next().unwrap();
        assert_eq!(nums_iter.next(), None);

        let x_range = left..left + width;
        let y_range = top..top + height;

        Self {
            id,
            x_range,
            y_range,
        }
    }
}

#[derive(Debug)]
struct Grid {
    x_ranges: Vec<Range<u16>>,
    y_ranges: Vec<Range<u16>>,
    tiles: Vec<Vec<u16>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn build(claims: &[Claim]) -> Self {
        let x_ranges = Self::calculate_ranges(claims.iter().map(|c| &c.x_range));
        let y_ranges = Self::calculate_ranges(claims.iter().map(|c| &c.y_range));

        let width = x_ranges.len();
        let height = y_ranges.len();

        let tiles = vec![vec![]; width * height];

        // blank grid
        let mut grid = Self {
            x_ranges,
            y_ranges,
            tiles,
            width,
            height,
        };

        // actually insert the claim ids now
        for claim in claims {
            grid.insert(claim);
        }

        grid
    }

    fn calculate_ranges<'a, I>(input: I) -> Vec<Range<u16>>
    where
        I: Iterator<Item = &'a Range<u16>>,
    {
        let mut splits = BTreeSet::new();
        for range in input {
            splits.insert(range.start);
            splits.insert(range.end);
        }

        let splits_vec: Vec<u16> = splits.into_iter().collect();
        splits_vec.windows(2).map(|v| (v[0]..v[1])).collect()
    }

    fn range_indices(ranges: &Vec<Range<u16>>, target: &Range<u16>) -> RangeInclusive<usize> {
        let start_index = ranges
            .binary_search_by_key(&target.start, |r| r.start)
            .unwrap();
        let end_index = ranges.binary_search_by_key(&target.end, |r| r.end).unwrap();
        start_index..=end_index
    }

    fn tile_index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    fn insert(&mut self, claim: &Claim) {
        let x_indices = Self::range_indices(&self.x_ranges, &claim.x_range);
        let y_indices = Self::range_indices(&self.y_ranges, &claim.y_range);

        for y in y_indices {
            for x in x_indices.clone() {
                let i = self.tile_index(x, y);
                self.tiles[i].push(claim.id);
            }
        }
    }

    fn area_with_overlap(&self) -> usize {
        let mut area = 0;

        for y in 0..self.height {
            let y_range = &self.y_ranges[y];
            for x in 0..self.width {
                let x_range = &self.x_ranges[x];
                let i = self.tile_index(x, y);
                let tile = &self.tiles[i];
                if tile.len() >= 2 {
                    area += x_range.len() * y_range.len();
                }
            }
        }

        area
    }

    fn is_non_overlapping(&self, claim: &Claim) -> bool {
        let x_indices = Self::range_indices(&self.x_ranges, &claim.x_range);
        let y_indices = Self::range_indices(&self.y_ranges, &claim.y_range);

        for y in y_indices {
            for x in x_indices.clone() {
                let i = self.tile_index(x, y);
                let tile = &self.tiles[i];
                if tile.len() >= 2 {
                    return false;
                }
            }
        }

        true
    }
}

fn part1(input: &str) -> usize {
    let claims = Claim::parse_list(input);
    let grid = Grid::build(&claims);
    grid.area_with_overlap()
}

fn part2(input: &str) -> u16 {
    let claims = Claim::parse_list(input);
    let grid = Grid::build(&claims);
    let non_overlapping: Vec<&Claim> = claims
        .iter()
        .filter(|c| grid.is_non_overlapping(c))
        .collect();
    assert_eq!(non_overlapping.len(), 1);
    non_overlapping.first().unwrap().id
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILE: &'static str = "example.txt";

    #[test]
    fn test_part1_example() {
        let result = part1(&read_file(EXAMPLE_FILE));
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_file(INPUT_FILE));
        assert_eq!(result, 98005);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(&read_file(EXAMPLE_FILE));
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_file(INPUT_FILE));
        assert_eq!(result, 331);
    }
}
