use crate::file::*;

pub fn run() {
    let input = parse(&read_input_file!());
    println!("part 1 result: {:?}", part1(&input));
    println!("part 2 result: {:?}", part2(&input));
}

struct Solver<'a> {
    chars: &'a [char],
    len: usize,
    to_ignore: Option<(char, char)>,
}

impl Solver<'_> {
    fn length_after_reduction(chars: &[char], ignore: Option<&char>) -> usize {
        let len = chars.len();
        let to_ignore = ignore.map(|c: &char| (c.to_ascii_lowercase(), c.to_ascii_uppercase()));
        let solver = Solver {
            chars,
            len,
            to_ignore,
        };
        solver.solve()
    }

    fn solve(&self) -> usize {
        let mut alive = vec![];
        let mut i = self.advance_if_needed(0);
        let mut j = self.advance(i);

        while j < self.len {
            if Self::should_react(&self.chars[i], &self.chars[j]) {
                // reduction, need to remove the two current chars
                match alive.pop() {
                    Some(prev) => {
                        // normal case, we  have something to go back to
                        i = prev; // go back to previous char on the left
                        j = self.advance(j); // advance to next char on the right
                    }
                    None => {
                        // special case, reducing stuff on the left side, jump ahead
                        i = self.advance(j);
                        j = self.advance(i);
                    }
                }
            } else {
                // no reduction, advance to next char and store last seen left
                alive.push(i);
                i = j;
                j = self.advance(j);
            }
        }
        alive.len() + 1 // add one for the straggler at the end
    }

    fn advance(&self, i: usize) -> usize {
        self.advance_if_needed(i + 1)
    }

    fn advance_if_needed(&self, start: usize) -> usize {
        let mut i = start;
        while i < self.len && self.should_ignore(&self.chars[i]) {
            i += 1;
        }
        i
    }

    fn should_react(l: &char, r: &char) -> bool {
        // 32 is diff between upper and lower case in ascii table
        (*l as i16 - *r as i16).abs() == 32
    }

    fn should_ignore(&self, c: &char) -> bool {
        match &self.to_ignore {
            Some((l, u)) => c == l || c == u,
            None => false,
        }
    }
}

fn parse(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn part1(chars: &[char]) -> usize {
    Solver::length_after_reduction(chars, None)
}

fn part2(chars: &[char]) -> usize {
    ('a'..='z')
        .map(|ignore| Solver::length_after_reduction(&chars, Some(&ignore)))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FILE: &'static str = "example.txt";

    #[test]
    fn test_part1_examples() {
        let result = part1(&parse(&read_example_file!()));
        assert_eq!(result, 10);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&parse(&read_input_file!()));
        assert_eq!(result, 11476);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(&parse(&read_example_file!()));
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&parse(&read_input_file!()));
        assert_eq!(result, 5446);
    }
}
