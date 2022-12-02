use std::fs;

// use itertools::Itertools;
// use lazy_static::lazy_static;
// use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Elves {
    inventories: Vec<Inventory>,
}

impl Elves {
    fn parse(input: &str) -> Elves {
        let inventories = input
            .split("\n\n")
            .map(|part| Inventory::parse(part))
            .collect();
        Elves {
            inventories: inventories,
        }
    }

    fn execute(&self) -> usize {
        self.inventories
            .iter()
            .map(|inv| inv.total_calories())
            .max()
            .unwrap()
    }
}

#[derive(Debug)]
struct Inventory {
    items: Vec<usize>,
}

impl Inventory {
    fn parse(input: &str) -> Inventory {
        let items = input.lines().map(|line| line.parse().unwrap()).collect();
        Inventory { items: items }
    }

    fn total_calories(&self) -> usize {
        self.items.iter().sum()
    }
}

fn part1(input: &str) -> usize {
    let data = Elves::parse(input);
    println!("{:?}", data);
    data.execute()
}

// fn part2(input: &str) -> usize {
//     let data = Elves::parse(input);
//     println!("{:?}", data);
//     data.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 66186);
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
