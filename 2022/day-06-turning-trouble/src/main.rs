use std::collections::HashSet;
use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

fn find_start_of_packet_marker(input: &str) -> Option<usize> {
    find_marker(input, 4)
}

fn find_start_of_message_marker(input: &str) -> Option<usize> {
    find_marker(input, 14)
}

fn find_marker(input: &str, message_length: usize) -> Option<usize> {
    (message_length..input.len()).find(|r| {
        let l = r - message_length;
        all_different_chars(&input[l..*r])
    })
}

fn all_different_chars(input: &str) -> bool {
    input.len() == input.chars().collect::<HashSet<char>>().len()
}

fn part1(input: &str) -> usize {
    find_start_of_packet_marker(input).unwrap()
}

fn part2(input: &str) -> usize {
    find_start_of_message_marker(input).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    static EXAMPLE2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    static EXAMPLE3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    static EXAMPLE4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    static EXAMPLE5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part1_examples() {
        assert_eq!(part1(EXAMPLE1), 7);
        assert_eq!(part1(EXAMPLE2), 5);
        assert_eq!(part1(EXAMPLE3), 6);
        assert_eq!(part1(EXAMPLE4), 10);
        assert_eq!(part1(EXAMPLE5), 11);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 1538);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(part2(EXAMPLE1), 19);
        assert_eq!(part2(EXAMPLE2), 23);
        assert_eq!(part2(EXAMPLE3), 23);
        assert_eq!(part2(EXAMPLE4), 29);
        assert_eq!(part2(EXAMPLE5), 26);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 2315);
    }
}
