use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::ops::RangeInclusive;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file(), "departure"));
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

        let fields = TicketSolver::parse_fields(fields_str);
        let your_ticket = TicketSolver::parse_ticket(your_ticket_str);
        let nearby_tickets = TicketSolver::parse_nearby_tickets(nearby_tickets_str);

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

    fn invalid_ticket_values(&self) -> Vec<usize> {
        return self.nearby_tickets.iter().map(|t| self.invalid_ticket_value(t)).filter(|o| o.is_some()).map(|o| *o.unwrap()).collect();
    }

    fn invalid_ticket_value<'a>(&self, ticket: &'a Ticket) -> Option<&'a usize> {
        return ticket.iter().find(|v| self.is_ticket_value_invalid(v));
    }

    fn is_ticket_value_invalid(&self, value: &usize) -> bool {
        return self.fields.iter().all(|(_, constraint)| !TicketSolver::satisfies_constraint(value, constraint));
    }

    fn satisfies_constraint(value: &usize, constraint: &FieldConstraints) -> bool {
        let (r1, r2) = constraint;
        return r1.contains(value) || r2.contains(value);
    }

    fn solve_fields(&self) -> HashMap<String, usize> {
        let mut valid_tickets: Vec<&Ticket> = self.nearby_tickets.iter().filter(|t| self.invalid_ticket_value(t).is_none()).collect();
        valid_tickets.push(&self.your_ticket);

        let mut field_mappings: HashMap<String, usize> = HashMap::new();

        let mut remaining_possible_mappings: HashMap<usize, HashSet<&String>> = (0..self.your_ticket.len()).map(|p| (p, self.possible_keys_for_position(&p, &valid_tickets))).collect();

        while !&remaining_possible_mappings.is_empty() {
            // find next position to deal with (min number of keys)
            let pos = TicketSolver::position_to_remove(&remaining_possible_mappings);
            let keys = remaining_possible_mappings.remove(&pos).unwrap();

            // unpack the matching key
            if keys.len() != 1 {
                panic!("Don't know how to handle a mapping of not exactly one possibility: {:?}", keys);
            }
            let key = keys.iter().next().unwrap().to_string();

            // filter key out of other sets
            for (_, set) in &mut remaining_possible_mappings {
                set.remove(&key);
            }

            // add mapping to result
            field_mappings.insert(key, pos);
        }

        return field_mappings;
    }

    fn position_to_remove(remaining_mappings: &HashMap<usize, HashSet<&String>>) -> usize {
        let (pos, _) = remaining_mappings.iter().min_by_key(|(_, set)| set.len()).unwrap();
        return *pos;
    }

    fn possible_keys_for_position(&self, position: &usize, tickets: &Vec<&Ticket>) -> HashSet<&String> {
        return self.fields.keys().filter(|key| self.key_valid_for_position(key, &position, &tickets)).collect();
    }

    fn key_valid_for_position(&self, key: &String, position: &usize, tickets: &Vec<&Ticket>) -> bool {
        let constraint = &self.fields[key];
        return tickets.iter().all(|t| TicketSolver::satisfies_constraint(&t[*position], constraint));
    }
}

fn part1(input: &str) -> usize {
    let solver = TicketSolver::parse(input);
    let invalid_ticket_values = solver.invalid_ticket_values();
    return invalid_ticket_values.iter().fold(0, |acc, x| acc + x);
}

fn part2(input: &str, wanted_field_prefix: &str) -> usize {
    let solver = TicketSolver::parse(input);
    let field_mappings = solver.solve_fields();
    let keys = solver.fields.keys().filter(|s| s.starts_with(wanted_field_prefix));
    let value_indices = keys.map(|k| field_mappings[k]);
    let your_ticket_values = value_indices.map(|i| solver.your_ticket[i]);
    return your_ticket_values.fold(1, |acc, x| acc * x);
}

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

    static EXAMPLE2: &str = indoc! {"    
        class: 0-1 or 4-19
        row: 0-5 or 8-19
        seat: 0-13 or 16-19

        your ticket:
        11,12,13

        nearby tickets:
        3,9,18
        15,1,5
        5,14,9
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 71);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 25059);
    }

    #[test]
    fn test_part2_example2_solver() {
        let solver = TicketSolver::parse(EXAMPLE2);
        let field_mappings = solver.solve_fields();
        let solution: HashMap<String, usize> = vec![
            ("row".to_string(), 0),
            ("class".to_string(), 1),
            ("seat".to_string(), 2),
        ].into_iter().collect();
        assert_eq!(field_mappings, solution);
    }

    #[test]
    fn test_part2_example2() {
        let result = part2(EXAMPLE2, "");
        assert_eq!(result, 1716);
    }    

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file(), "departure");
        assert_eq!(result, 3253972369789);
    }
}