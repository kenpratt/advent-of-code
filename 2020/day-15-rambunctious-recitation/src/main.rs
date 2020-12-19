use std::collections::HashMap;

use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file(), 2020));
    println!("part 2 result: {:?}", part2(&read_input_file(), 30000000));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct MemoryGame {
    starting_numbers: Vec<usize>,
}

impl MemoryGame {
    fn parse(input: &str) -> MemoryGame {
        let starting_numbers = input.trim().split(",").map(|s| s.parse::<usize>().unwrap()).collect();
        return MemoryGame {
            starting_numbers: starting_numbers,
        }
    }

    fn execute(&self, num_turns: usize) -> usize {
        let mut spoken = HashMap::new();

        let mut last = Turn {
            turn: 0,
            num: 0,
            previously_spoken: None,
        };

        for num in &self.starting_numbers {
            let turn = last.turn + 1;
            spoken.insert(*num, turn);

            last = Turn {
                turn: turn,
                num: *num,
                previously_spoken: None,
            };
            //println!("{:?}", last);
        }

        while last.turn < num_turns {
            let turn = last.turn + 1;

            let num = match last.previously_spoken {
                Some(turns_ago) => turns_ago,
                None => 0,
            };

            let previously_spoken = match spoken.get(&num) {
                Some(lost_spoken_at_turn) => Some(turn - lost_spoken_at_turn),
                None => None,
            };

            spoken.insert(num, turn);

            last = Turn {
                turn: turn,
                num: num,
                previously_spoken: previously_spoken,
            };
            //println!("{:?}", last);
        }

        return last.num;
    }
}

#[derive(Debug)]
struct Turn {
    turn: usize,
    num: usize,
    previously_spoken: Option<usize>,
}

fn part1(input: &str, num_turns: usize) -> usize {
    let game = MemoryGame::parse(input);
    return game.execute(num_turns);
}

fn part2(input: &str, num_turns: usize) -> usize {
    let game = MemoryGame::parse(input);
    return game.execute(num_turns);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "0,3,6";
    static EXAMPLE2: &str = "1,3,2";
    static EXAMPLE3: &str = "2,1,3";
    static EXAMPLE4: &str = "1,2,3";
    static EXAMPLE5: &str = "2,3,1";
    static EXAMPLE6: &str = "3,2,1";
    static EXAMPLE7: &str = "3,1,2";

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1, 2020);
        assert_eq!(result, 436);
    }

    #[test]
    fn test_part1_example2() {
        let result = part1(EXAMPLE2, 2020);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part1_example3() {
        let result = part1(EXAMPLE3, 2020);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_part1_example4() {
        let result = part1(EXAMPLE4, 2020);
        assert_eq!(result, 27);
    }

    #[test]
    fn test_part1_example5() {
        let result = part1(EXAMPLE5, 2020);
        assert_eq!(result, 78);
    }

    #[test]
    fn test_part1_example6() {
        let result = part1(EXAMPLE6, 2020);
        assert_eq!(result, 438);
    }

    #[test]
    fn test_part1_example7() {
        let result = part1(EXAMPLE7, 2020);
        assert_eq!(result, 1836);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file(), 2020);
        assert_eq!(result, 276);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1, 30000000);
        assert_eq!(result, 175594);
    }

    #[test]
    fn test_part2_example2() {
        let result = part2(EXAMPLE2, 30000000);
        assert_eq!(result, 2578);
    }

    #[test]
    fn test_part2_example3() {
        let result = part2(EXAMPLE3, 30000000);
        assert_eq!(result, 3544142);
    }

    #[test]
    fn test_part2_example4() {
        let result = part2(EXAMPLE4, 30000000);
        assert_eq!(result, 261214);
    }

    #[test]
    fn test_part2_example5() {
        let result = part2(EXAMPLE5, 30000000);
        assert_eq!(result, 6895259);
    }

    #[test]
    fn test_part2_example6() {
        let result = part2(EXAMPLE6, 30000000);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part2_example7() {
        let result = part2(EXAMPLE7, 30000000);
        assert_eq!(result, 362);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file(), 30000000);
        assert_eq!(result, 31916);
    }
}