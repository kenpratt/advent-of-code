use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
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

    fn solve_for_allergens(&self) -> HashMap<String, String> {
        Solver::new(&self.foods).solve()
    }

    fn allergen_free_ingredient_count(&self) -> usize {
        let allergen_mappings = self.solve_for_allergens();
        let allergen_ingredients: HashSet<&String> = allergen_mappings.values().collect();

        let mut ingredient_counts: HashMap<&String, usize> = HashMap::new();
        for food in &self.foods {
            for ingredient in &food.ingredients {
                let count = ingredient_counts.entry(ingredient).or_insert(0);
                *count += 1;
            }
        }

        ingredient_counts.into_iter().filter(|(k, _)| {
            !allergen_ingredients.contains(k)
        }).fold(0, |acc, (_, v)| acc + v)
    }

    fn allergen_ingredients(&self) -> Vec<String> {
        let allergen_mappings = self.solve_for_allergens();

        // sort by allergen name
        let mut allergens: Vec<&String> = allergen_mappings.keys().collect();
        allergens.sort();

        allergens.into_iter().map(|a| allergen_mappings.get(a).unwrap().clone()).collect()
    }
}

#[derive(Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl Food {
    fn parse(input: &str) -> Food {
        lazy_static! {
            static ref ID_RE: Regex = Regex::new(r"\A([a-z\s]+) \(contains ([a-z,\s]+)\)\z").unwrap();
        }
        let captures = ID_RE.captures(input).unwrap();
        let ingredients_str = captures.get(1).unwrap().as_str();
        let allergens_str = captures.get(2).unwrap().as_str();

        let ingredients: HashSet<String> = ingredients_str.split(' ').map(|s| s.to_string()).collect();
        let allergens: HashSet<String> = allergens_str.split(", ").map(|s| s.to_string()).collect();

        return Food {
            ingredients: ingredients,
            allergens: allergens,
        }
    }
}

#[derive(Debug)]
struct Solver<'a> {
    foods: &'a Vec<Food>,
}

impl Solver<'_> {
    fn new(foods: &Vec<Food>) -> Solver {
        Solver {
            foods: foods,
        }
    }

    fn solve(&self) -> HashMap<String, String> {
        let mut allergen_possibilites = Solver::possible_ingredients_for_allergens(self.foods);

        let mut solution = HashMap::new();

        while !allergen_possibilites.is_empty() {
            // find an allergen that can only map to a single ingredient
            let allergen = allergen_possibilites.iter().find(|(_, s)| s.len() == 1).unwrap().0.clone();

            // find the ingredient that it maps to
            let ingredient: String = allergen_possibilites.remove(&allergen).unwrap().into_iter().next().unwrap();

            // remove that ingredients from the available options for other allergens
            for (_, ingredients) in &mut allergen_possibilites {
                ingredients.remove(&ingredient);
            }

            solution.insert(allergen, ingredient);
        }

        solution
    }

    fn possible_ingredients_for_allergens(foods: &Vec<Food>) -> HashMap<String, HashSet<String>> {
        // for each allergen:
        // - find foods that contain that allergen
        // - find intersection of all ingredients in those foods (to get
        //   possible ingredients that the allergen could map to)
        let allergens = foods.iter().fold(HashSet::new(), |acc, food| {
            acc.union(&food.allergens).cloned().collect()
        });

        allergens.into_iter().map(|allergen| {
            let foods_with_allergen: Vec<&Food> = foods.iter().filter(|f| {
                f.allergens.contains(&allergen)
            }).collect();

            let ingredients_with_allergen: HashSet<String> = foods_with_allergen[1..].iter().fold(foods_with_allergen[0].ingredients.clone(), |acc, food| {
                acc.intersection(&food.ingredients).cloned().collect()
            });

            (allergen, ingredients_with_allergen)
        }).collect()
    }
}

fn part1(input: &str) -> usize {
    let data = Data::parse(input);
    data.allergen_free_ingredient_count()
}

fn part2(input: &str) -> String {
    let data = Data::parse(input);
    return data.allergen_ingredients().join(",");
}

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
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 2556);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, "mxmxvkd,sqjhc,fvjkl".to_string());
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, "vcckp,hjz,nhvprqb,jhtfzk,mgkhhc,qbgbmc,bzcrknb,zmh".to_string());
    }
}