use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"\A(.*) \| (.*)\z").unwrap();
}

fn parse(input: &str) -> Vec<Entry> {
    input.lines().map(|line| Entry::parse(line)).collect()
}

type Pattern = String;

#[derive(Debug)]
struct Entry {
    signals: Vec<Pattern>,
    outputs: Vec<Pattern>,
}

impl Entry {
    fn parse(input: &str) -> Entry {
        let captures = LINE_RE.captures(input).unwrap();
        let signals_str = captures.get(1).unwrap().as_str();
        let outputs_str = captures.get(2).unwrap().as_str();
        let signals = signals_str
            .split_whitespace()
            .map(|s| Entry::parse_pattern(s))
            .collect();
        let outputs = outputs_str
            .split_whitespace()
            .map(|s| Entry::parse_pattern(s))
            .collect();
        Entry {
            signals: signals,
            outputs: outputs,
        }
    }

    fn parse_pattern(input: &str) -> Pattern {
        input.to_string()
    }
}

fn part1(input: &str) -> usize {
    let entries = parse(input);
    println!("{:?}", entries);
    entries
        .iter()
        .map(|e| {
            e.outputs
                .iter()
                .filter(|o| o.len() < 5 || o.len() > 6)
                .count()
        })
        .sum()
}

// fn part2(input: &str) -> usize {
//     let data = Data::parse(input);
//     println!("{:?}", data);
//     data.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
    "};

    static EXAMPLE2: &str = indoc! {"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part1_example2() {
        let result = part1(EXAMPLE2);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 375);
    }

    // #[test]
    // fn test_part2_example1() {
    //     let result = part2(EXAMPLE1);
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}
