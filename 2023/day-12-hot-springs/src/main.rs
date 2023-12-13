use std::{cmp, fs};

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
    if count == 0 {
        panic!("Unreachable");
    }

    match group.len() {
        0 => {
            // no group to satisfy the count
            vec![]
        }
        1 => {
            match &group[0] {
                (Condition::Damaged, num_damaged) => {
                    if *num_damaged == count {
                        // what a nice coincidence, there's exactly one damaged group of exactly the right size
                        // consume the full group
                        vec![GroupResult::Consumed(true)]
                    } else {
                        // wrong number of damaged in this position, no solution
                        vec![]
                    }
                }
                (Condition::Unknown, num_unknown) => {
                    let mut results = vec![
                        // special case, treat all unknowns as operational and consume the group
                        // but do not satisfy the count
                        GroupResult::Consumed(false),
                    ];

                    if *num_unknown >= count {
                        // we have enough unknowns to satisfy this count.
                        // but there may be multiple ways to consume the unknown portion
                        for index in 0..=(num_unknown - count) {
                            let num_extra = num_unknown - count - index;
                            if num_extra > 1 {
                                // we can leave a remainder
                                results.push(GroupResult::Remainder(vec![(
                                    Condition::Unknown,
                                    num_extra - 1, // leave one as a divider
                                )]));
                            } else {
                                // consume the whole roup
                                results.push(GroupResult::Consumed(true));
                            }
                        }
                    } else {
                        // not enough unknowns to meet this criteria, no solution
                    };

                    results
                }
                _ => panic!("Unreachable"),
            }
        }
        _ => {
            match (&group[0], &group[1]) {
                ((Condition::Damaged, num_damaged), (Condition::Unknown, num_unknown)) => {
                    if *num_damaged == count {
                        // what a nice coincidence, there's exactly one damaged group of exactly the right size
                        // return the rest of the group as the remainder, reducing unknowns by one to leave a divider.
                        if *num_unknown > 1 {
                            let mut rem_group = vec![(Condition::Unknown, num_unknown - 1)];
                            rem_group.append(&mut group[2..].to_owned());
                            vec![GroupResult::Remainder(rem_group)]
                        } else {
                            // consume the unknown chunk too
                            if group.len() > 2 {
                                vec![GroupResult::Remainder(group[2..].to_owned())]
                            } else {
                                vec![GroupResult::Consumed(true)]
                            }
                        }
                    } else if *num_damaged < count {
                        let need_more = count - num_damaged;
                        if *num_unknown > need_more {
                            // have enough unknowns to solve this and leave a gap
                            let num_extra = num_unknown - need_more;
                            if num_extra > 1 {
                                // leave a remainder
                                let mut rem_group = vec![(Condition::Unknown, num_extra - 1)];
                                rem_group.append(&mut group[2..].to_owned());
                                vec![GroupResult::Remainder(rem_group)]
                            } else {
                                // consume the unknown chunk too
                                if group.len() > 2 {
                                    vec![GroupResult::Remainder(group[2..].to_owned())]
                                } else {
                                    vec![GroupResult::Consumed(true)]
                                }
                            }
                        } else if *num_unknown == need_more {
                            // this scenario is weird, we need exactly damaged + unknows, which only works
                            // if it's at the end of the group. if more damaged follow, we're hooped.
                            if group.get(2).is_none() {
                                vec![GroupResult::Consumed(true)]
                            } else {
                                vec![] // no solution
                            }
                        } else {
                            // consume both the damaged and unknown sections, and recur on the rest
                            solve_group(group[2..].to_owned(), count - num_damaged - num_unknown)
                        }
                    } else {
                        // we have more damaged than we want => no solution
                        vec![]
                    }
                }
                ((Condition::Unknown, num_unknown), (Condition::Damaged, num_damaged)) => {
                    // there are two possible approaches:
                    let mut results = vec![];

                    // 1) try to use the unknowns for a match, leaving space for the damaged to be remainder
                    if *num_unknown > count {
                        for index in 0..=(num_unknown - count - 1) {
                            let num_extra = num_unknown - count - index;
                            assert!(num_extra > 0);

                            let mut rem_group = vec![];
                            if num_extra > 1 {
                                rem_group.push((Condition::Unknown, num_extra - 1));
                            }
                            rem_group.append(&mut group[1..].to_owned());

                            results.push(GroupResult::Remainder(rem_group));
                        }
                    }

                    // 2) use the right side of the unknowns to "glom on" to the damaged, with a special
                    // case of glomming zero on, ignoring the unknown group
                    if *num_damaged <= count {
                        let want_more_damaged = count - num_damaged;
                        for num_to_glom in 0..=cmp::min(*num_unknown, want_more_damaged) {
                            // recur so we can handle the logic of leaving a separator after the damaged group
                            let mut recur_group =
                                vec![(Condition::Damaged, num_damaged + num_to_glom)];
                            recur_group.append(&mut group[2..].to_owned());
                            let mut recur_results = solve_group(recur_group, count);
                            results.append(&mut recur_results);
                        }
                    }

                    results
                }
                _ => panic!("Unreachable"),
            }
        }
    }
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
