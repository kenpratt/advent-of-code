use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use indoc::indoc;
//use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(read_input_file()));
    //println!("part 2 result: {:?}", part2(read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct Ruleset {
    rules: Vec<Rule>,
}

impl Ruleset {
    fn parse(input: &str) -> Ruleset {
        let rules = input.lines().map(|line| Rule::parse(line)).collect();
        return Ruleset {
            rules: rules,
        }
    }

    fn can_hold(&self, kind: &str) -> HashSet<&str> {
        // find rules that can directly hold this kind of bag
        let subset: HashSet<&str> = self.rules.iter().filter(|r| r.can_hold(kind)).map(|r| r.kind.as_str()).collect();

        // recur on these kinds, to find rules that can indirectly hold this kind of bag
        let output = subset.clone();
        return subset.iter().map(|kind| self.can_hold(kind)).fold(output, |acc, x| acc.union(&x).cloned().collect());
    }
}

#[derive(Debug)]
struct Rule {
    kind: String,
    contents: HashMap<String, usize>,
}

impl Rule {
    fn parse(input: &str) -> Rule {
        // dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        let re = Regex::new(r"^(.+) bags contain (.+)\.$").unwrap();
        let captures = re.captures(input).unwrap();
        let kind = captures.get(1).unwrap().as_str();
        let contents_str = captures.get(2).unwrap().as_str();

        let contents = contents_str.split(", ").map(|s| Rule::parse_contents(s)).collect();

        return Rule {
            kind: kind.to_string(),
            contents: contents,
        }
    }
    
    fn parse_contents(s: &str) -> (String, usize) {
        let re = Regex::new(r"^(\d+|no) (.+) bags?$").unwrap();
        let captures = re.captures(s).unwrap();
        let quantity_str = captures.get(1).unwrap().as_str();
        let kind = captures.get(2).unwrap().as_str();

        let quantity = match quantity_str {
            "no" => 0,
            _ => quantity_str.parse::<usize>().unwrap(),
        };

        return (kind.to_string(), quantity);
    }

    fn can_hold(&self, kind: &str) -> bool {
        return self.contents.contains_key(kind);
    }
}

fn part1(input: String) -> usize {
    let ruleset = Ruleset::parse(&input);
    let subset = ruleset.can_hold("shiny gold");
    return subset.len();
}

// fn part2(input: String) -> usize {
//     let data = Ruleset::parse(input);
//     return data.execute();
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = indoc! {"
            light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.
        "};
        let result = part1(input.to_string());
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(
            read_input_file()
        );
        assert_eq!(result, 302);
    }

    // #[test]
    // fn test_part2_example1() {
    //     let result = part2(
    //         "".to_string()
    //     );
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(
    //         read_input_file()
    //     );
    //     assert_eq!(result, 0);
    // }
}