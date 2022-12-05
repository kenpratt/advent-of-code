use std::fs;

use std::collections::HashSet;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct RucksackList {
    rucksacks: Vec<Rucksack>,
}

impl RucksackList {
    fn parse(input: &str) -> RucksackList {
        let rucksacks = input.lines().map(|line| Rucksack::parse(line)).collect();
        RucksackList { rucksacks }
    }

    fn sum_priorites_for_common_items(&self) -> usize {
        self.rucksacks
            .iter()
            .map(|r| r.common_item_priority())
            .sum()
    }
}

#[derive(Debug)]
struct Rucksack {
    first_compartment: HashSet<char>,
    second_compartment: HashSet<char>,
}

impl Rucksack {
    fn parse(input: &str) -> Rucksack {
        assert!(input.len() % 2 == 0);
        let midpoint = input.len() / 2;
        let first_compartment = input[0..midpoint].chars().collect();
        let second_compartment = input[midpoint..].chars().collect();
        Rucksack {
            first_compartment,
            second_compartment,
        }
    }

    fn common_items(&self) -> HashSet<&char> {
        self.first_compartment
            .intersection(&self.second_compartment)
            .collect()
    }

    fn common_item_priority(&self) -> usize {
        let items = self.common_items();
        assert_eq!(items.len(), 1);
        let item = items.iter().next().unwrap();
        Self::priority(item)
    }

    fn priority(item: &char) -> usize {
        let n = *item as usize;
        match n {
            // a..z
            97..=122 => (n - 97) + 1, // a=1
            // A..Z
            65..=90 => (n - 65) + 27, // A=27
            _ => panic!("Unexpected char: {}", item),
        }
    }
}

fn part1(input: &str) -> usize {
    let rucksacks = RucksackList::parse(input);
    println!("{:?}", rucksacks);
    rucksacks.sum_priorites_for_common_items()
}

// fn part2(input: &str) -> usize {
//     let rucksacks = RucksackList::parseinput);
//     println!("{:?}", rucksacks);
//     rucksacks.sum_priorites_for_common_items()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 157);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 7597);
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
