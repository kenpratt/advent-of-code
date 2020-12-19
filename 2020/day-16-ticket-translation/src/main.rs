use std::collections::HashMap;
use std::fs;
use std::ops::RangeInclusive;

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
struct TicketSolver {
    fields: HashMap<String, FieldConstraints>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

type FieldConstraints = (RangeInclusive<usize>, RangeInclusive<usize>);
type Ticket = Vec<usize>;

impl TicketSolver {
    fn parse(input: &str) -> TicketSolver {
        lazy_static! {
            static ref PARTS_RE: Regex = Regex::new(r"(?m)\A([\s\S]+)your ticket:([\s\S]+)nearby tickets:([\s\S]+)\z").unwrap();
        }

        let captures = PARTS_RE.captures(input).unwrap();
        let fields_str = captures.get(1).unwrap().as_str().trim();
        let your_ticket_str = captures.get(2).unwrap().as_str().trim();
        let nearby_tickets_str = captures.get(3).unwrap().as_str().trim();

        // println!("{:?}", fields_str);
        // println!("{:?}", your_ticket_str);
        // println!("{:?}", nearby_tickets_str);

        let fields = TicketSolver::parse_fields(fields_str);
        let your_ticket = TicketSolver::parse_ticket(your_ticket_str);
        let nearby_tickets = TicketSolver::parse_nearby_tickets(nearby_tickets_str);

        println!("{:?}", fields);
        println!("{:?}", your_ticket);
        println!("{:?}", nearby_tickets);

        return TicketSolver {
            fields: fields,
            your_ticket: your_ticket,
            nearby_tickets: nearby_tickets,
        }
    }

    fn parse_fields(input: &str) -> HashMap<String, FieldConstraints> {
        return input.lines().map(|s| TicketSolver::parse_field(s)).collect();
    }

    fn parse_field(input: &str) -> (String, FieldConstraints) {
        lazy_static! {
            static ref FIELD_RE: Regex = Regex::new(r"\A([a-z\s]+): (\d+)-(\d+) or (\d+)-(\d+)\z").unwrap();
        }

        let captures = FIELD_RE.captures(input).unwrap();
        let name = captures.get(1).unwrap().as_str();
        let r1 = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let r2 = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let r3 = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
        let r4 = captures.get(5).unwrap().as_str().parse::<usize>().unwrap();

        return (name.to_string(), ((r1..=r2), (r3..=r4)));
    }

    fn parse_ticket(input: &str) -> Ticket {
        return input.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
    }

    fn parse_nearby_tickets(input: &str) -> Vec<Ticket> {
        return input.lines().map(|l| TicketSolver::parse_ticket(l)).collect();
    }

    fn execute(&self) -> usize {
        return 0;
    }
}

fn part1(input: &str) -> usize {
    let solver = TicketSolver::parse(input);
    return solver.execute();
}

// fn part2(input: &str) -> usize {
//     let solver = TicketSolver::parse(input);
//     return solver.execute();
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50

        your ticket:
        7,1,14

        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12
    "};    

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 71);
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