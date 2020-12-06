use std::collections::HashSet;
use std::fs;

use indoc::indoc;

fn main() {
    println!("part 1 result: {:?}", part1(read_input_file()));
    println!("part 2 result: {:?}", part2(read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

fn parse_input(input: &str) -> Vec<Group> {
    return input.split("\n\n").map(|chunk| Group::parse(chunk)).collect();
}

#[derive(Debug)]
struct Group {
    people: Vec<String>,
}

impl Group {
    fn parse(input: &str) -> Group {
        return Group {
            people: input.lines().map(|l| l.to_string()).collect(),
        }
    }
    
    fn union_size(&self) -> usize {
        let chars_union: HashSet<char> = self.people.join("").chars().collect();
        return chars_union.len();
    }
    
    fn intersection_size(&self) -> usize {        
        let base: HashSet<char> = self.people[0].chars().collect();
        let intersection = self.people[1..].iter().fold(base, |acc, x| acc.intersection(&x.chars().collect()).cloned().collect());
        return intersection.len();
    }
}

fn part1(input: String) -> usize {
    let groups = parse_input(&input);
    return groups.iter().map(|g| g.union_size()).fold(0, |acc, x| acc + x);
}

fn part2(input: String) -> usize {
    let groups = parse_input(&input);
    return groups.iter().map(|g| g.intersection_size()).fold(0, |acc, x| acc + x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = indoc! {"
            abc

            a
            b
            c
            
            ab
            ac
            
            a
            a
            a
            a
            
            b
        "};
        let result = part1(input.to_string());
        assert_eq!(result, 11);
    }    

    #[test]
    fn test_part1_solution() {
        let result = part1(
            read_input_file()
        );
        assert_eq!(result, 6748);
    }

    #[test]
    fn test_part2_example1() {
        let input = indoc! {"
            abc

            a
            b
            c
            
            ab
            ac
            
            a
            a
            a
            a
            
            b
        "};
        let result = part2(input.to_string());
        assert_eq!(result, 6);
    }    

    #[test]
    fn test_part2_solution() {
        let result = part2(
            read_input_file()
        );
        assert_eq!(result, 3445);
    }
}