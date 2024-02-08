use crate::file;

use std::fmt::Debug;

const INPUT_FILE: &'static str = "input.txt";
const EXAMPLE_FILE: &'static str = "example.txt";

// logic in common between the traits
macro_rules! aoc_common {
    () => {
        const FILE: &'static str;

        fn parse(input: String) -> I;

        fn parse_input_file() -> I {
            Self::parse_file(INPUT_FILE)
        }

        fn parse_example_file() -> I {
            Self::parse_file(EXAMPLE_FILE)
        }

        fn parse_file(filename: &str) -> I {
            let input_str = Self::read_file(filename);
            Self::parse(input_str)
        }

        fn read_file(filename: &str) -> String {
            file::read_file(filename, Self::FILE)
        }

        // for tests
        fn parse_str(input: &str) -> I {
            Self::parse(input.to_owned())
        }
    };
}

pub trait AoC<I, R1, R2>
where
    R1: Debug,
    R2: Debug,
{
    aoc_common!();

    fn part1(input: &I) -> R1;
    fn part2(input: &I) -> R2;

    fn run(print: bool) {
        let input = Self::parse_input_file();

        let res1 = Self::part1(&input);
        if print {
            println!("part 1 result: {:?}", res1);
        }

        let res2 = Self::part2(&input);
        if print {
            println!("part 2 result: {:?}", res2);
        }
    }
}

pub trait AoCWithParams<I, P1, P2, R1, R2>
where
    R1: Debug,
    R2: Debug,
{
    aoc_common!();

    const PARAMS_PART1: P1;
    const PARAMS_PART2: P2;

    fn part1(input: &I, params: P1) -> R1;
    fn part2(input: &I, params: P2) -> R2;

    fn run(print: bool) {
        let input = Self::parse_input_file();

        let res1 = Self::part1(&input, Self::PARAMS_PART1);
        if print {
            println!("part 1 result: {:?}", res1);
        }

        let res2 = Self::part2(&input, Self::PARAMS_PART2);
        if print {
            println!("part 2 result: {:?}", res2);
        }
    }
}
