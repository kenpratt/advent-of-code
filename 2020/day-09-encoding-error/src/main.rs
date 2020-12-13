use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file(), 25));
    println!("part 2 result: {:?}", part2(&read_input_file(), 675280050));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct Data {
    numbers: Vec<usize>,
    preamble: usize,
    index: usize,
}

impl Data {
    fn parse(input: &str, preamble: usize) -> Data {
        let numbers = input.lines().map(|line| line.parse::<usize>().unwrap()).collect();
        return Data {
            numbers: numbers,
            preamble: preamble,
            index: 0,
        }
    }

    fn find_bad_number(&mut self) -> usize {
        loop {
            match self.step() {
                Some(bad_number) => return bad_number,
                None => {},
            }
        }
    }

    fn step(&mut self) -> Option<usize> {
        let target = self.numbers[self.index + self.preamble];
        if self.is_valid(target) {
            self.index += 1;
            return None;
        } else {
            return Some(target);
        }
    }

    fn is_valid(&self, target: usize) -> bool {
        for i in self.index..(self.index + self.preamble) {
            for j in self.index..(self.index + self.preamble) {
                if i != j {
                    if (self.numbers[i] + self.numbers[j]) == target {
                        return true;
                    }
                }
            }
        }

        return false;
    }

    fn find_contiguous_sum(&self, target: usize) -> usize {
        for i in 0..self.numbers.len() {
            let mut sum = 0;
            for j in i..self.numbers.len() {
                sum += self.numbers[j];
                if sum == target {
                    let range = &self.numbers[i..=j];
                    let min = range.iter().min().unwrap();
                    let max = range.iter().max().unwrap();
                    return min + max;
                } else if sum > target {
                    break;
                }
                // else continue
            }
        }

        panic!("couldn't find contiguous sum")
    }
}

fn part1(input: &str, preamble: usize) -> usize {
    let mut data = Data::parse(input, preamble);
    return data.find_bad_number();
}

fn part2(input: &str, target: usize) -> usize {
    let data = Data::parse(input, 0);
    return data.find_contiguous_sum(target);
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1, 5);
        assert_eq!(result, 127);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file(), 25);
        assert_eq!(result, 675280050);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1, 127);
        assert_eq!(result, 62);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file(), 675280050);
        assert_eq!(result, 96081673);
    }
}