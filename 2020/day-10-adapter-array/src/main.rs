use std::fs;

use indoc::indoc;
// use lazy_static::lazy_static;
// use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct Data {
    joltages: Vec<usize>,
}

impl Data {
    fn parse(input: &str) -> Data {
        let mut joltages: Vec<usize> = input.lines().map(|line| line.parse::<usize>().unwrap()).collect();

        joltages.sort();
        joltages.insert(0, 0);
        joltages.push(joltages.last().unwrap() + 3);
        println!("{:?}", joltages);

        return Data {
            joltages: joltages,
        }
    }

    fn differences(&self) -> Vec<usize> {
        let mut differences: Vec<usize> = vec![0, 0, 0, 0];

        for i in 0..(self.joltages.len()-1) {
            let diff = self.joltages[i+1] - self.joltages[i];
            differences[diff] += 1;
        }

        println!("{:?}", differences);
        return differences;
    }

    fn count_combinations(&self) -> usize {
        let seqs = self.sequence_lengths();
        println!("{:?}", seqs);

        let factors = seqs.iter().map(|s| Data::factor_for_sequence_length(s));
        return factors.fold(1, |acc, x| acc * x);
    }

    fn sequence_lengths(&self) -> Vec<usize> {
        let mut sequences = vec![];

        let mut seq = 0;
        for i in 0..(self.joltages.len()-1) {
            let diff = self.joltages[i+1] - self.joltages[i];
            match diff {
                1 => {
                    seq += 1;
                },
                3 => {
                    if seq > 0 {
                        sequences.push(seq);
                    }
                    seq = 0;
                },
                _ => {
                    panic!("Don't understand diffs other than 1 and 3");
                }
            }
        }

        return sequences;         
    }

    fn factor_for_sequence_length(seq: &usize) -> usize {
        return match seq {
            0 | 1 => 1,
            2 => 2,
            3 => 4,
            4 => 7,
            _ => panic!("Don't know how to calculate factor for sequence length"),
        }        
    }
}

fn part1(input: &str) -> usize {
    let data = Data::parse(input);
    let differences = data.differences();
    return differences[1] * differences[3];
}

fn part2(input: &str) -> usize {
    let data = Data::parse(input);
    return data.count_combinations();
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = indoc! {"
        16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4
    "};

    static EXAMPLE2: &str = indoc! {"
        28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part1_example2() {
        let result = part1(EXAMPLE2);
        assert_eq!(result, 220);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 1917);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2_example2() {
        let result = part2(EXAMPLE2);
        assert_eq!(result, 19208);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 113387824750592);
    }
}