use std::{collections::HashSet, fs};

// use itertools::Itertools;
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
struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl Schematic {
    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref NUMBER_RE: Regex = Regex::new(r"\d+").unwrap();
            static ref SYMBOL_RE: Regex = Regex::new(r"[\*\/\=\+\%\@\#\&\-\$]").unwrap();
        }

        let mut numbers = vec![];
        let mut symbols = vec![];
        for (y, line) in input.lines().enumerate() {
            for m in NUMBER_RE.find_iter(line) {
                let x = m.start();
                let length = m.len();
                let value = m.as_str().parse::<usize>().unwrap();

                let position = Coord {
                    x: x as isize,
                    y: y as isize,
                };
                let number = Number {
                    value,
                    length,
                    position,
                };
                numbers.push(number);
            }

            for m in SYMBOL_RE.find_iter(line) {
                let x = m.start();
                let value = m.as_str().chars().next().unwrap();

                let position = Coord {
                    x: x as isize,
                    y: y as isize,
                };
                let symbol = Symbol { value, position };
                symbols.push(symbol);
            }
        }
        Self { numbers, symbols }
    }

    fn symbol_positions(&self) -> HashSet<Coord> {
        self.symbols.iter().map(|s| s.position).collect()
    }

    fn part_numbers(&self) -> Vec<Number> {
        let sym = self.symbol_positions();
        self.numbers
            .iter()
            .cloned()
            .filter(|n| n.is_part_number(&sym))
            .collect()
    }

    fn gear_ratios(&self) -> Vec<usize> {
        let parts = self.part_numbers();
        let parts_with_neighbours: Vec<(Number, HashSet<Coord>)> =
            parts.into_iter().map(|p| (p, p.neighbours())).collect();

        self.symbols
            .iter()
            .filter(|s| s.value == '*')
            .flat_map(|s| s.gear_ratio(&parts_with_neighbours))
            .collect()
    }
}

#[derive(Clone, Copy, Debug)]
struct Number {
    value: usize,
    length: usize,
    position: Coord,
}

impl Number {
    fn neighbours(&self) -> HashSet<Coord> {
        let mut out = HashSet::new();

        let l = self.position.x - 1;
        let r = self.position.x + self.length as isize;

        out.insert(Coord {
            x: l,
            y: self.position.y,
        });
        out.insert(Coord {
            x: r,
            y: self.position.y,
        });

        for x in l..=r {
            out.insert(Coord {
                x: x,
                y: self.position.y - 1,
            });
            out.insert(Coord {
                x: x,
                y: self.position.y + 1,
            });
        }

        out
    }

    fn is_part_number(&self, symbol_positions: &HashSet<Coord>) -> bool {
        self.neighbours()
            .iter()
            .any(|p| symbol_positions.contains(p))
    }
}

#[derive(Debug)]
struct Symbol {
    value: char,
    position: Coord,
}

impl Symbol {
    fn gear_ratio(&self, parts_with_neighbours: &[(Number, HashSet<Coord>)]) -> Option<usize> {
        let pos = &self.position;
        let adjacent_parts: Vec<usize> = parts_with_neighbours
            .iter()
            .filter(|(_n, neighbours)| neighbours.contains(pos))
            .map(|(n, _)| n.value)
            .collect();
        if adjacent_parts.len() == 2 {
            let ratio = adjacent_parts[0] * adjacent_parts[1];
            Some(ratio)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Coord {
    x: isize,
    y: isize,
}

fn part1(input: &str) -> usize {
    let schematic = Schematic::parse(input);
    schematic.part_numbers().iter().map(|p| p.value).sum()
}

fn part2(input: &str) -> usize {
    let schematic = Schematic::parse(input);
    schematic.gear_ratios().iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 527144);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 467835);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 81463996);
    }
}
