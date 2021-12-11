use std::fs;
use std::str::Chars;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
enum ParseResult {
    Legal,
    Incomplete(Vec<char>),
    Corrupted(char),
}

#[derive(Debug)]
struct Parser<'a> {
    chars: Chars<'a>,
    open_chunks: Vec<char>,
}

impl Parser<'_> {
    fn parse(line: &str) -> ParseResult {
        let mut parser = Parser {
            chars: line.chars(),
            open_chunks: vec![],
        };
        parser.run()
    }

    fn run(&mut self) -> ParseResult {
        loop {
            let result = self.consume_char();
            if result.is_some() {
                return result.unwrap();
            }
        }
    }

    fn consume_char(&mut self) -> Option<ParseResult> {
        match self.chars.next() {
            Some(c) => match c {
                '(' | '[' | '{' | '<' => self.open_chunk(c),
                '>' | '}' | ']' | ')' => self.close_chunk(c),
                _ => panic!("Invalid char: {}", c),
            },
            None => {
                // we got to the end of the line. if we have nothing dangling, it's a legal line.
                if self.open_chunks.len() == 0 {
                    Some(ParseResult::Legal)
                } else {
                    let completion_chars = self.completion_chars();
                    Some(ParseResult::Incomplete(completion_chars))
                }
            }
        }
    }

    fn completion_chars(&self) -> Vec<char> {
        self.open_chunks
            .iter()
            .map(|c| Parser::closing_delimiter(c))
            .rev()
            .collect()
    }

    fn open_chunk(&mut self, c: char) -> Option<ParseResult> {
        self.open_chunks.push(c);
        None
    }

    fn close_chunk(&mut self, c: char) -> Option<ParseResult> {
        let expected_opening_delimiter = Parser::opening_delimiter(&c);
        if self.open_chunks.last() == Some(&expected_opening_delimiter) {
            self.open_chunks.pop();
            None
        } else {
            Some(ParseResult::Corrupted(c))
        }
    }

    fn opening_delimiter(c: &char) -> char {
        match c {
            '>' => '<',
            '}' => '{',
            ']' => '[',
            ')' => '(',
            _ => panic!("Invalid char: {}", c),
        }
    }

    fn closing_delimiter(c: &char) -> char {
        match c {
            '<' => '>',
            '{' => '}',
            '[' => ']',
            '(' => ')',
            _ => panic!("Invalid char: {}", c),
        }
    }
}

fn part1(input: &str) -> usize {
    let results: Vec<ParseResult> = input.lines().map(|line| Parser::parse(line)).collect();
    println!("{:?}", results);
    results
        .iter()
        .filter_map(|r| part1_score_for_result(r))
        .sum()
}

fn part1_score_for_result(result: &ParseResult) -> Option<usize> {
    match result {
        ParseResult::Legal | ParseResult::Incomplete(_) => None,
        ParseResult::Corrupted(c) => Some(score_for_corrupting_char(c)),
    }
}

fn score_for_corrupting_char(c: &char) -> usize {
    match c {
        '>' => 25137,
        '}' => 1197,
        ']' => 57,
        ')' => 3,
        _ => panic!("Invalid char: {}", c),
    }
}

fn part2(input: &str) -> usize {
    let results: Vec<ParseResult> = input.lines().map(|line| Parser::parse(line)).collect();
    println!("{:?}", results);
    let mut scores: Vec<usize> = results
        .iter()
        .filter_map(|r| part2_score_for_result(r))
        .collect();
    println!("{:?}", scores);
    scores.sort();
    scores[scores.len() / 2]
}

fn part2_score_for_result(result: &ParseResult) -> Option<usize> {
    match result {
        ParseResult::Legal => None,
        ParseResult::Incomplete(completion_chars) => {
            Some(score_for_completion_chars(&completion_chars))
        }
        ParseResult::Corrupted(_) => None,
    }
}

fn score_for_completion_chars(chars: &[char]) -> usize {
    chars
        .iter()
        .fold(0, |acc, c| acc * 5 + score_for_completion_char(c))
}

fn score_for_completion_char(c: &char) -> usize {
    match c {
        '>' => 4,
        '}' => 3,
        ']' => 2,
        ')' => 1,
        _ => panic!("Invalid char: {}", c),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 26397);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 390993);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 288957);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 2391385187);
    }
}
