use std::fmt;
use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

// there are duplicate values, so to do the mixing, use a vec including
// the indices of the original array, so that the values are unique.
type Element = (usize, i16);

#[derive(Clone, Debug)]
struct Sequence(Vec<Element>);

impl Sequence {
    fn parse(input: &str) -> Self {
        let values = input
            .lines()
            .map(|line| line.parse::<i16>().unwrap())
            .enumerate()
            .collect();
        Self(values)
    }

    fn mix(&self) -> Self {
        let mut mixing = self.clone();

        // iterate in original order
        for element in &self.0 {
            mixing.move_element(element);
        }
        mixing
    }

    fn position(&self, element: &Element) -> usize {
        self.0.iter().position(|e| e == element).unwrap()
    }

    fn find_value(&self, value: i16) -> usize {
        self.0.iter().position(|e| e.1 == value).unwrap()
    }

    fn values(&self) -> Vec<i16> {
        self.0.iter().map(|(_i, v)| *v).collect()
    }

    fn move_element(&mut self, element: &Element) {
        let amount = element.1;
        if amount == 0 {
            return;
        }

        let curr_index = self.position(element);

        // remove element before calculating new index, as if it wraps it'll
        // "skip itself"
        let e = self.0.remove(curr_index);

        // calculate new index
        let new_index = self.shifted_index(curr_index, amount);

        // add it back
        self.0.insert(new_index, e);
    }

    fn shifted_index(&self, curr_index: usize, shift: i16) -> usize {
        let len = self.0.len() as i16;
        let mut new_index = (curr_index as i16) + shift;
        while new_index < 0 {
            new_index += len;
        }
        (new_index % len) as usize
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, v) in self.0.iter().enumerate() {
            write!(f, "[{:04}] {:?}\n", i, v)?;
        }
        Ok(())
    }
}

fn part1(input: &str) -> i16 {
    let sequence = Sequence::parse(input);
    let decrypted = sequence.mix();
    let zero_index = decrypted.find_value(0);

    [1000, 2000, 3000]
        .iter()
        .map(|offset| decrypted.shifted_index(zero_index, *offset as i16))
        .map(|i| decrypted.0[i].1)
        .sum()
}

// fn part2(input: &str) -> usize {
//     let items = Data::parse(input);
//     dbg!(&items);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        1
        2
        -3
        3
        -2
        0
        4
    "};

    static CASE1: &str = indoc! {"
        1
        2
        -10
        3
        -2
        0
        11
    "};

    static CASE2: &str = indoc! {"
        0
        4
        5
        7
        8
    "};

    static CASE3: &str = indoc! {"
        -4
        -5
        0
        -7
        -8
    "};

    static CASE4: &str = indoc! {"
        101
        -52
        4
        88
        -11
    "};

    static CASE5: &str = indoc! {"
    3
    4
    5
    6
    7
"};

    #[test]
    fn test_example_decrypt() {
        let result = Sequence::parse(EXAMPLE).mix();
        assert_eq!(result.values(), vec![-2, 1, 2, -3, 4, 0, 3]);
    }

    #[test]
    fn test_case1_decrypt() {
        let result = Sequence::parse(CASE1).mix();
        assert_eq!(result.values(), vec![1, -2, 2, -10, 0, 11, 3]);
    }

    #[test]
    fn test_case2_decrypt() {
        let result = Sequence::parse(CASE2).mix();
        assert_eq!(result.values(), vec![8, 0, 7, 4, 5]);
    }

    #[test]
    fn test_case3_decrypt() {
        let result = Sequence::parse(CASE3).mix();
        assert_eq!(result.values(), vec![-8, -7, -5, -4, 0]);
    }

    #[test]
    fn test_case4_decrypt() {
        let result = Sequence::parse(CASE4).mix();
        assert_eq!(result.values(), vec![-52, -11, 101, 4, 88]);
    }

    #[test]
    fn test_case5_decrypt() {
        let result = Sequence::parse(CASE5).mix();
        assert_eq!(result.values(), vec![4, 5, 3, 7, 6]);
    }

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 15297);
    }

    // #[test]
    // fn test_part2_example() {
    //     let result = part2(EXAMPLE);
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}
