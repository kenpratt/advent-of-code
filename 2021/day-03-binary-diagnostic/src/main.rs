use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

fn parse(input: &str) -> (Vec<usize>, usize) {
    let lines: Vec<&str> = input.lines().collect();
    let digits = lines[0].len();
    let values = lines
        .iter()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect();
    (values, digits)
}

fn is_bit_set(input: &usize, pos: &usize) -> bool {
    input & (1 << pos) != 0
}

fn bit_set_in_majority(values: &[usize], position: &usize) -> bool {
    let num_set = values.iter().filter(|&v| is_bit_set(v, position)).count();
    let half = (values.len() + 1) / 2; // round up odd numbers
    num_set >= half
}

fn part1(input: &str) -> usize {
    let (values, digits) = parse(input);
    let gamma_rate = (0..digits).fold(0, |acc, pos| {
        let over_half = bit_set_in_majority(&values, &pos);
        let bit_to_set = (over_half as usize) << pos;
        acc | bit_to_set
    });
    let epsilon_rate = (0..digits).fold(0, |acc, pos| {
        let set_in_gamma = is_bit_set(&gamma_rate, &pos);
        let bit_to_set = (!set_in_gamma as usize) << pos;
        acc | bit_to_set
    });
    println!("{:?} {:?}", gamma_rate, epsilon_rate);
    gamma_rate * epsilon_rate
}

fn build_rating(values: &Vec<usize>, digits: usize, use_majority: bool) -> usize {
    let mut remaining: Vec<usize> = values.clone();
    for pos in (0..digits).rev() {
        if remaining.len() > 1 {
            let set_in_majority = bit_set_in_majority(&remaining, &pos);
            let target = if use_majority {
                set_in_majority
            } else {
                !set_in_majority
            };
            remaining = remaining
                .into_iter()
                .filter(|v| is_bit_set(v, &pos) == target)
                .collect();
        }
    }
    assert_eq!(remaining.len(), 1);
    remaining[0]
}

fn part2(input: &str) -> usize {
    let (values, digits) = parse(input);
    let oxygen_generator_rating = build_rating(&values, digits, true);
    let co2_scrubber_rating = build_rating(&values, digits, false);
    println!("{:?} {:?}", oxygen_generator_rating, co2_scrubber_rating);
    oxygen_generator_rating * co2_scrubber_rating
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 198);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 4174964);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 230);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 4474944);
    }
}
