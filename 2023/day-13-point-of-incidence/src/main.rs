use std::{cmp, fs};

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Pattern {
    regular: Grid,
    rotated: Grid,
}

impl Pattern {
    fn parse_list(input: &str) -> Vec<Self> {
        input
            .split("\n\n")
            .map(|chunk| Self::parse(chunk))
            .collect()
    }

    fn parse(input: &str) -> Self {
        let regular = Grid::parse(input);
        let rotated = regular.rotated();
        Self { regular, rotated }
    }

    fn split_value(&self) -> usize {
        match self.regular.find_split() {
            Some(v) => v,
            None => match self.rotated.find_split() {
                Some(v) => v * 100,
                None => panic!("No split found"),
            },
        }
    }

    fn split_value_smudged(&self) -> usize {
        match self.regular.find_split_smudged() {
            Some(v) => v,
            None => match self.rotated.find_split_smudged() {
                Some(v) => v * 100,
                None => panic!("No split found"),
            },
        }
    }
}

#[derive(Debug)]
struct Grid {
    values: Vec<Vec<bool>>,
    reversed: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(values: Vec<Vec<bool>>) -> Self {
        let width = values[0].len();
        let height = values.len();

        let reversed = values
            .iter()
            .map(|row| row.iter().rev().cloned().collect())
            .collect();

        Self {
            values,
            reversed,
            width,
            height,
        }
    }

    fn parse(input: &str) -> Self {
        Self::new(
            input
                .lines()
                .map(|row| {
                    row.chars()
                        .map(|c| match c {
                            '#' => true,
                            '.' => false,
                            _ => panic!("Unexpected input: {}", c),
                        })
                        .collect::<Vec<bool>>()
                })
                .collect(),
        )
    }

    fn rotated(&self) -> Self {
        Self::new(
            (0..self.width)
                .map(|x| (0..self.height).map(|y| self.values[y][x]).collect())
                .collect(),
        )
    }

    fn find_split(&self) -> Option<usize> {
        // can't mirror at first index, but can mirror at last index as I'm defining mirror point as split before index
        (1..self.width).find(|split_at| {
            let rev_split_at = self.width - split_at;
            let compare_n = cmp::min(*split_at, rev_split_at);

            // ensure every row is mirrored
            (0..self.height).all(|y| {
                let reg_split = &self.values[y][*split_at..(split_at + compare_n)];
                let rev_split = &self.reversed[y][rev_split_at..(rev_split_at + compare_n)];
                reg_split == rev_split
            })
        })
    }

    fn find_split_smudged(&self) -> Option<usize> {
        // can't mirror at first index, but can mirror at last index as I'm defining mirror point as split before index
        (1..self.width).find(|split_at| {
            let rev_split_at = self.width - split_at;
            let compare_n = cmp::min(*split_at, rev_split_at);

            // ensure every row is mirrored
            let bad_rows: Vec<(&[bool], &[bool])> = (0..self.height)
                .map(|y| {
                    let reg_split = &self.values[y][*split_at..(split_at + compare_n)];
                    let rev_split = &self.reversed[y][rev_split_at..(rev_split_at + compare_n)];
                    (reg_split, rev_split)
                })
                .filter(|(reg_split, rev_split)| reg_split != rev_split)
                .collect();

            // smudged image should only have one bad line
            if bad_rows.len() == 1 {
                let (reg_split, rev_split) = &bad_rows[0];
                assert_eq!(reg_split.len(), rev_split.len());
                let num_conflicts = (0..reg_split.len())
                    .filter(|i| reg_split[*i] != rev_split[*i])
                    .count();
                num_conflicts == 1 // smudge should have exactly one conflict
            } else {
                false
            }
        })
    }
}

fn part1(input: &str) -> usize {
    let patterns = Pattern::parse_list(input);
    patterns.iter().map(|p| p.split_value()).sum()
}

fn part2(input: &str) -> usize {
    let patterns = Pattern::parse_list(input);
    patterns.iter().map(|p| p.split_value_smudged()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 405);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 43614);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 400);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 36771);
    }
}
