use std::fs;

use itertools::Itertools;
use memoize::memoize;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Record {
    springs: Vec<Condition>,
    damaged_counts: Vec<usize>,
}

impl Record {
    fn parse_list(input: &str, multiply_by: usize) -> Vec<Self> {
        input
            .lines()
            .map(|line| Self::parse(line, multiply_by))
            .collect()
    }

    fn parse(input: &str, multiply_by: usize) -> Self {
        let mut parts = input.split_whitespace();

        let springs_str = Self::multiply_input(parts.next().unwrap(), multiply_by, "?");
        let counts_str = Self::multiply_input(parts.next().unwrap(), multiply_by, ",");

        let springs = Condition::parse_list(&springs_str);
        let damaged_counts: Vec<usize> = counts_str
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        assert_eq!(parts.next(), None);

        Self {
            springs,
            damaged_counts,
        }
    }

    fn multiply_input(input: &str, multiply_by: usize, join_on: &str) -> String {
        if multiply_by > 1 {
            (0..multiply_by).map(|_i| input.to_owned()).join(join_on)
        } else {
            input.to_owned()
        }
    }

    fn num_arrangements(&self) -> usize {
        let groups: Vec<Vec<(Condition, usize)>> = self
            .springs
            .split(|c| *c == Condition::Operational)
            .filter(|group| !group.is_empty())
            .map(|group| Condition::chunked(group))
            .collect();

        solve(groups, self.damaged_counts.clone())
    }
}

#[memoize]
fn solve(groups: Vec<Vec<(Condition, usize)>>, counts: Vec<usize>) -> usize {
    match (groups.is_empty(), counts.is_empty()) {
        (true, true) => return 1,  // valid solution
        (true, false) => return 0, // remaining counts but no groups to satisfy them, invalid
        (false, true) => {
            if groups
                .iter()
                .all(|group| group.len() == 1 && group[0].0 == Condition::Unknown)
            {
                // no counts, but all that's left are one or more groups of unknowns, so that's a valid solution
                return 1; // exactly one solution
            } else {
                return 0; // no solution, we have some damaged groups left
            }
        }
        (false, false) => (), // continue
    }

    let need_chars = counts.iter().sum::<usize>() + (counts.len() - 1);
    let have_chars = groups
        .iter()
        .map(|g| g.iter().map(|(_c, n)| n).sum::<usize>())
        .sum::<usize>()
        + (groups.len() - 1);
    if need_chars > have_chars {
        // don't have enough stuff, can halt this branch now
        return 0;
    }

    let count = &counts[0];
    let group = &groups[0];

    let group_results = solve_group(group.to_owned(), *count);
    group_results
        .into_iter()
        .map(|group_result| {
            match group_result {
                // group was consumed, but count was not
                GroupResult::Consumed(false) => solve(groups[1..].to_owned(), counts.clone()),

                // group was consumed, and so was count
                GroupResult::Consumed(true) => {
                    solve(groups[1..].to_owned(), counts[1..].to_owned())
                }

                // count was satisfied, with a remainder
                GroupResult::Remainder(remainder) => {
                    let mut recur_groups = vec![remainder];
                    recur_groups.append(&mut groups[1..].to_owned());
                    solve(recur_groups, counts[1..].to_owned())
                }
            }
        })
        .sum()
}

#[memoize]
fn solve_group(group: Vec<(Condition, usize)>, count: usize) -> Vec<GroupResult> {
    // possible solutions that solve the count
    let mut res = solve_group_impl(false, count, group.clone());

    // special case - if we have a single unknown group, can skip the whole thing
    // without satisfying the count
    if group.len() == 1 && group[0].0 == Condition::Unknown {
        res.push(GroupResult::Consumed(false));
    }

    res
}

#[memoize]
fn solve_group_impl(
    in_damaged_sequence: bool,
    remaining_count: usize,
    chunks: Vec<(Condition, usize)>,
) -> Vec<GroupResult> {
    if chunks.is_empty() {
        if remaining_count == 0 {
            vec![GroupResult::Consumed(true)]
        } else {
            vec![] // ran out of stuff; no solution
        }
    } else if remaining_count == 0 {
        match &chunks[0] {
            (Condition::Damaged, _num_damaged) => {
                vec![] // no room to leave a gap for the remainder; no solution
            }
            (Condition::Unknown, num_unknown) => {
                if *num_unknown == 1 {
                    if chunks[1..].is_empty() {
                        vec![GroupResult::Consumed(true)]
                    } else {
                        vec![GroupResult::Remainder(chunks[1..].to_owned())]
                    }
                } else if *num_unknown > 1 {
                    let rem = prepend_chunk((Condition::Unknown, *num_unknown - 1), &chunks[1..]);
                    vec![GroupResult::Remainder(rem)]
                } else {
                    vec![] // no room to leave a gap for the remainder; no solution
                }
            }
            _ => panic!("Unreachable"),
        }
    } else {
        match &chunks[0] {
            (Condition::Damaged, num_damaged) => {
                if remaining_count >= *num_damaged {
                    // use the whole chunk
                    solve_group_impl(true, remaining_count - num_damaged, chunks[1..].to_owned())
                } else {
                    vec![] // too many damaged to satisfy count; no solution
                }
            }
            (Condition::Unknown, num_unknown) => {
                // no matter what, we can try a damaged sequnce at the beginning
                let mut res = if remaining_count >= *num_unknown {
                    // use the whole chunk
                    solve_group_impl(true, remaining_count - *num_unknown, chunks[1..].to_owned())
                } else {
                    // potentially leave a remainder
                    let rem = prepend_chunk(
                        (Condition::Unknown, *num_unknown - remaining_count),
                        &chunks[1..],
                    );
                    solve_group_impl(false, 0, rem)
                };

                // if we aren't already in a damaged sequence, we variations starting later in the chunk
                if !in_damaged_sequence {
                    // try skipping part of the chunk
                    for num_to_skip in 1..*num_unknown {
                        let rem = prepend_chunk(
                            (Condition::Unknown, *num_unknown - num_to_skip),
                            &chunks[1..],
                        );
                        let mut other_res = solve_group_impl(true, remaining_count, rem);
                        res.append(&mut other_res);
                    }

                    // and also try skipping the whole chunk
                    let mut other_res =
                        solve_group_impl(true, remaining_count, chunks[1..].to_owned());
                    res.append(&mut other_res);
                }

                res
            }
            _ => panic!("Unreachable"),
        }
    }
}

fn prepend_chunk(
    chunk: (Condition, usize),
    other: &[(Condition, usize)],
) -> Vec<(Condition, usize)> {
    let mut res = vec![chunk];
    res.append(&mut other.to_owned());
    res
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum GroupResult {
    Consumed(bool),
    Remainder(Vec<(Condition, usize)>),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Condition {
    fn parse_list(input: &str) -> Vec<Self> {
        input.chars().map(|c| Self::parse(&c)).collect()
    }

    fn parse(input: &char) -> Self {
        use Condition::*;

        match input {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => panic!("Unknown condition: {}", input),
        }
    }

    fn chunked(input: &[Condition]) -> Vec<(Condition, usize)> {
        input
            .iter()
            .dedup_with_count()
            .map(|(n, c)| (*c, n))
            .collect()
    }
}

fn part1(input: &str) -> usize {
    let records = Record::parse_list(input, 1);
    records.iter().map(|r| r.num_arrangements()).sum()
}

fn part2(input: &str) -> usize {
    let records = Record::parse_list(input, 5);
    records.iter().map(|r| r.num_arrangements()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    "};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("???.### 1,1,3"), 1);
        assert_eq!(part1(".??..??...?##. 1,1,3"), 4);
        assert_eq!(part1("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(part1("????.#...#... 4,1,1"), 1);
        assert_eq!(part1("????.######..#####. 1,6,5"), 4);
        assert_eq!(part1("?###???????? 3,2,1"), 10);

        let result = part1(EXAMPLE);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part1_custom() {
        assert_eq!(part1("?#???#?? 3,3"), 3); //.###.### or ###.###. or ###..###
        assert_eq!(part1("##???? 3,1"), 2); // ###.#. or ###..#
    }

    #[test]
    fn test_part1_selections() {
        assert_eq!(part1("???????#???#?? 1,3,3,3"), 5);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 7047);
    }

    #[test]
    fn test_part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 525152);
    }

    #[test]
    fn test_part2_selections() {
        assert_eq!(part2("??????????..?.?? 3,4,1"), 166249531);
        assert_eq!(part2("????#?.????.??? 1,2,1,2"), 8037281657);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 17391848518844);
    }
}
