use std::fs;

static LOOP_DIVISOR: usize = 20201227;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Data {
    public_keys: Vec<usize>,
}

impl Data {
    fn parse(input: &str) -> Data {
        let public_keys = input.lines().map(|line| line.parse::<usize>().unwrap()).collect();
        Data {
            public_keys: public_keys,
        }
    }

    fn calculate_encryption_key(&self) -> usize {
        assert_eq!(self.public_keys.len(), 2);
        let public_key0 = self.public_keys[0];
        let public_key1 = self.public_keys[1];

        let loop_size0 = determine_loop_size(7, public_key0);
        let loop_size1 = determine_loop_size(7, public_key1);

        let encryption_key0 = calculate_encryption_key(public_key1, loop_size0);
        let encryption_key1 = calculate_encryption_key(public_key0, loop_size1);
        assert_eq!(encryption_key0, encryption_key1);

        encryption_key0
    }
}

fn determine_loop_size(subject_number: usize, public_key: usize) -> usize {
    let mut loop_size: usize = 0;
    let mut value: usize = 1;
    loop {
        loop_size += 1;
        value = (value * subject_number) % LOOP_DIVISOR;
        if value == public_key {
            return loop_size;
        }
    }
}

fn calculate_encryption_key(public_key: usize, loop_size: usize) -> usize {
    let mut value: usize = 1;
    for _ in 0..loop_size {
        value = (value * public_key) % LOOP_DIVISOR;
    }
    value
}

fn part1(input: &str) -> usize {
    let data = Data::parse(input);
    data.calculate_encryption_key()
}

// fn part2(input: &str) -> usize {
//     let data = Data::parse(input);
//     data.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        5764801
        17807724
    "};

    #[test]
    fn test_determine_loop_size() {
        assert_eq!(determine_loop_size(7, 5764801), 8);
        assert_eq!(determine_loop_size(7, 17807724), 11);
    }

    #[test]
    fn test_calculate_encryption_key() {
        assert_eq!(calculate_encryption_key(17807724, 8), 14897079);
        assert_eq!(calculate_encryption_key(5764801, 11), 14897079);
    }

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 14897079);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 11707042);
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