use std::fs;

// use lazy_static::lazy_static;
// use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct Data {
    earliest_time: usize,
    shuttle_ids: Vec<usize>,
}

impl Data {
    fn parse(input: &str) -> Data {
        let lines: Vec<&str> = input.lines().collect();
        assert_eq!(lines.len(), 2);
        let earliest_time = lines[0].parse::<usize>().unwrap();
        let shuttle_ids = lines[1].split(',').filter(|s| s != &"x").map(|s| s.parse::<usize>().unwrap()).collect();

        return Data {
            earliest_time: earliest_time,
            shuttle_ids: shuttle_ids,
        }
    }

    fn earliest_available_shuttle(&self) -> (usize, usize) {
        return self.shuttle_ids.iter().map(|id| (*id, self.calculate_wait(id))).min_by_key(|p| p.1).unwrap();
    }

    fn calculate_wait(&self, shuttle_id: &usize) -> usize {
        return shuttle_id - self.earliest_time % shuttle_id;
    }
}

fn part1(input: &str) -> usize {
    let data = Data::parse(input);
    let (shuttle_id, wait) = data.earliest_available_shuttle();
    return shuttle_id * wait;
}

// fn part2(input: &str) -> usize {
//     let data = Data::parse(input);
//     return data.execute();
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        939
        7,13,x,x,59,x,31,19
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 295);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 205);
    }

    // #[test]
    // fn test_part2_example1() {
    //     let result = part2(EXAMPLE1);
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}