use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use indoc::indoc;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(read_input_file()));
    println!("part 2 result: {:?}", part2(read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct Ruleset {
    rules: HashMap<String, Rule>,
}

impl Ruleset {
    fn parse(input: &str) -> Ruleset {
        let rules = input.lines().map(|line| Rule::parse(line));
        return Ruleset {
            rules: rules.map(|r| (r.kind.clone(), r)).collect(),
        }
    }

    fn can_hold(&self, kind: &str) -> HashSet<&str> {
        // find rules that can directly hold this kind of bag
        let subset: HashSet<&str> = self.rules.values().filter(|r| r.can_hold(kind)).map(|r| r.kind.as_str()).collect();

        // recur on these kinds, to find rules that can indirectly hold this kind of bag
        let output = subset.clone();
        return subset.iter().map(|kind| self.can_hold(kind)).fold(output, |acc, x| acc.union(&x).cloned().collect());
    }

    fn num_bags_inside(&self, kind: &str) -> usize {
        let rule = self.rules.get(kind).unwrap();
        return rule.contents.iter().map(|(c_kind, c_num)| c_num + c_num * self.num_bags_inside(c_kind)).fold(0, |acc, x| acc + x);
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

        let contents = contents_str.split(", ").map(|s| Rule::parse_contents(s)).filter(|v| v.is_some()).map(|v| v.unwrap()).collect();

        return Rule {
            kind: kind.to_string(),
            contents: contents,
        }
    }
    
    fn parse_contents(s: &str) -> Option<(String, usize)> {
        if s == "no other bags" {
            return None;
        }

        let re = Regex::new(r"^(\d+) (.+) bags?$").unwrap();
        let captures = re.captures(s).unwrap();
        let quantity_str = captures.get(1).unwrap().as_str();
        let kind = captures.get(2).unwrap().as_str();

        let quantity = quantity_str.parse::<usize>().unwrap();
        return Some((kind.to_string(), quantity));
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

fn part2(input: String) -> usize {
    let ruleset = Ruleset::parse(&input);
    return ruleset.num_bags_inside("shiny gold");
}

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

    #[test]
    fn test_part2_example1() {
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
        let result = part2(input.to_string());
        assert_eq!(result, 32);
    }

    #[test]
    fn test_part2_example2() {
        let input = indoc! {"
            shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain 2 dark green bags.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags.
        "};
        let result = part2(input.to_string());
        assert_eq!(result, 126);
    }    

    #[test]
    fn test_part2_solution() {
        let result = part2(
            read_input_file()
        );
        assert_eq!(result, 4165);
    }
}