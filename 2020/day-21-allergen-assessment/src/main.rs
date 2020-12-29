use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct Data {
    foods: Vec<Food>,
}

impl Data {
    fn parse(input: &str) -> Data {
        let foods = input.lines().map(|line| Food::parse(line)).collect();
        return Data {
            foods: foods,
        }
    }

    fn allergen_free_ingredients(&self) -> usize {
        println!("{:?}", self);
        return 0;
    }
}

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl Food {
    fn parse(input: &str) -> Food {
        lazy_static! {
            static ref ID_RE: Regex = Regex::new(r"\A([a-z\s]+) \(contains ([a-z,\s]+)\)\z").unwrap();
        }
        let captures = ID_RE.captures(input).unwrap();
        let ingredients_str = captures.get(1).unwrap().as_str();
        let allergens_str = captures.get(2).unwrap().as_str();

        let ingredients: Vec<String> = ingredients_str.split(' ').map(|s| s.to_string()).collect();
        let allergens: Vec<String> = allergens_str.split(", ").map(|s| s.to_string()).collect();

        return Food {
            ingredients: ingredients,
            allergens: allergens,
        }
    }
}

fn part1(input: &str) -> usize {
    let data = Data::parse(input);
    let ingredients = data.allergen_free_ingredients();
    return 0;
}

// fn part2(input: &str) -> usize {
//     let data = Data::parse(input);
//     return data.execute();
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
        trh fvjkl sbzzf mxmxvkd (contains dairy)
        sqjhc fvjkl (contains soy)
        sqjhc mxmxvkd sbzzf (contains fish)
    "};    

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 0);
    }

    // #[test]
    // fn test_part1_solution() {
    //     let result = part1(&read_input_file());
    //     assert_eq!(result, 0);
    // }

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