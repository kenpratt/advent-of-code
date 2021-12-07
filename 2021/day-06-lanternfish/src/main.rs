use std::fs;

use cached::proc_macro::cached;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

fn parse(input: &str) -> Vec<u8> {
    let line = input.lines().next().unwrap();
    line.split(',').map(|s| s.parse::<u8>().unwrap()).collect()
}

// 80 rounds:
// - fish@0: 0, 7, 14, 21, 28, 35, 42, 49, 56, 63, 70, 77 (12)
// - fish@1: 1, 8, 15, 22, 29, 36, 43, 50, 57, 64, 71, 78 (12)
// - fish@2: 2, ..., 72, 79 (12)
// - fish@3: 3, ..., 73 (11)
// - fish@4: 4, ..., 74 (11)
// - fish@5: 5, ..., 75 (11)
// - fish@6: 6, ..., 76 (11)
fn count_fish(fish_timers: &[u8], rounds_remaining: &u16) -> usize {
    fish_timers
        .iter()
        .map(|f| count_fish_at_day_0((*rounds_remaining as isize) - (*f as isize)))
        .sum()
}

// use the cached crate to memoize the result
#[cached]
fn count_fish_at_day_0(rounds_remaining: isize) -> usize {
    if rounds_remaining <= 0 {
        return 1; // base case: count self
    }

    let mut offspring = 1; // count self
    let mut round = rounds_remaining;
    while round > 0 {
        offspring += count_fish_at_day_0(round - 9);
        round -= 7;
    }
    offspring
}

fn part1(input: &str) -> usize {
    let fish = parse(input);
    count_fish(&fish, &80)
}

fn part2(input: &str) -> usize {
    let fish = parse(input);
    count_fish(&fish, &256)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        3,4,3,1,2
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 5934);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 386755);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 26984457539);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 1732731810807);
    }
}
