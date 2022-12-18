use std::cmp::Ordering;
use std::fs;

use itertools::Itertools;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
enum Token {
    OpenParen,
    CloseParen,
    Number(u32),
}

const RADIX: u32 = 10;

struct Tokenizer {
    curr_num: Option<u32>,
    result: Vec<Token>,
}

impl Tokenizer {
    fn run(input: &str) -> Vec<Token> {
        let mut tokenizer = Self {
            curr_num: None,
            result: vec![],
        };
        tokenizer.run_(input);
        tokenizer.result
    }

    fn run_(&mut self, input: &str) {
        use Token::*;
        for c in input.chars() {
            match c {
                '[' => self.result.push(OpenParen),
                ',' => self.close_number(),
                ']' => {
                    self.close_number();
                    self.result.push(CloseParen)
                }
                '0'..='9' => self.add_number(c),
                _ => panic!("Unexpected char: {}", c),
            }
        }
    }

    fn add_number(&mut self, c: char) {
        let d = c.to_digit(RADIX).unwrap();
        self.curr_num = match self.curr_num {
            None => Some(d),
            Some(x) => Some(x * 10 + d),
        }
    }

    fn close_number(&mut self) {
        use Token::*;
        if self.curr_num.is_some() {
            self.result.push(Number(self.curr_num.unwrap()));
        }
        self.curr_num = None;
    }
}

type NodeList = Vec<Node>;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Node {
    List(NodeList),
    Number(u32),
}

impl Node {
    fn parse(input: &str) -> Self {
        let tokens = Tokenizer::run(input);
        let mut token_iter = tokens.into_iter();
        let res = Self::parse_node(&mut token_iter).unwrap();
        assert!(token_iter.next().is_none());
        res
    }

    fn parse_node<I: Iterator<Item = Token>>(tokens: &mut I) -> Option<Self> {
        match tokens.next().unwrap() {
            Token::OpenParen => {
                let mut res = vec![];
                loop {
                    match Self::parse_node(tokens) {
                        Some(n) => res.push(n),
                        None => break,
                    };
                }
                Some(Node::List(res))
            }
            Token::CloseParen => None,
            Token::Number(n) => Some(Node::Number(n)),
        }
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        use itertools::EitherOrBoth::{Both, Left, Right};
        use Node::*;
        use Ordering::*;

        match (self, other) {
            (Number(l), Number(r)) => l.cmp(r),
            (List(l), List(r)) => l
                .iter()
                .zip_longest(r)
                .find_map(|e| match e {
                    Both(li, ri) => match li.cmp(ri) {
                        Equal => None, // keep going
                        x => Some(x),  // stop
                    },
                    Left(_) => Some(Greater), // stop
                    Right(_) => Some(Less),   // stop
                })
                .unwrap_or(Equal), // both ran out of elements
            (Number(l), List(_)) => List(vec![Number(*l)]).cmp(other),
            (List(_), Number(r)) => self.cmp(&List(vec![Number(*r)])),
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct PacketPair {
    left: NodeList,
    right: NodeList,
}

impl PacketPair {
    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        assert_eq!(lines.len(), 2);
        let left = Self::parse_packet(lines[0]);
        let right = Self::parse_packet(lines[1]);
        Self { left, right }
    }

    fn parse_packet(input: &str) -> NodeList {
        match Node::parse(input) {
            Node::List(l) => l,
            Node::Number(_) => panic!("Not expecting a number: {}", input),
        }
    }

    fn in_order(&self) -> bool {
        self.left < self.right
    }
}

fn parse_packet_pairs(input: &str) -> Vec<PacketPair> {
    input
        .split("\n\n")
        .map(|chunk| PacketPair::parse(chunk))
        .collect()
}

fn part1(input: &str) -> usize {
    let pairs = parse_packet_pairs(input);
    pairs
        .iter()
        .enumerate()
        .filter(|(_, p)| p.in_order())
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(input: &str) -> usize {
    let divider_packets = vec![
        vec![Node::List(vec![Node::Number(2)])],
        vec![Node::List(vec![Node::Number(6)])],
    ];

    let mut packets = divider_packets.clone();
    let pairs = parse_packet_pairs(input);
    for pair in pairs {
        packets.push(pair.left);
        packets.push(pair.right);
    }
    packets.sort();

    divider_packets
        .iter()
        .map(|d| packets.iter().position(|p| p == d).unwrap() + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        [1,1,3,1,1]
        [1,1,5,1,1]
        
        [[1],[2,3,4]]
        [[1],4]
        
        [9]
        [[8,7,6]]
        
        [[4,4],4,4]
        [[4,4],4,4,4]
        
        [7,7,7,7]
        [7,7,7]
        
        []
        [3]
        
        [[[]]]
        [[]]
        
        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 5852);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 140);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 24190);
    }
}
