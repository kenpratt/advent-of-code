use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct History {
    layers: Vec<Vec<isize>>,
}

impl History {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        let readings = parse_numbers(input);
        let layers = Self::build_initial_layers(&readings);

        Self { layers }
    }

    fn build_initial_layers(readings: &Vec<isize>) -> Vec<Vec<isize>> {
        let mut layers = vec![readings.clone()];

        while !all_zero(layers.last().unwrap()) {
            let last: &Vec<isize> = layers.last().unwrap();
            let this: Vec<isize> = last.windows(2).map(|s| s[1] - s[0]).collect();
            layers.push(this);
        }

        layers
    }

    fn extrapolate(&mut self) -> isize {
        // add 0 to bottom layer to bootstrap
        self.layers.last_mut().unwrap().push(0);

        for i in (0..(self.layers.len() - 1)).rev() {
            let v = self.layers[i].last().unwrap() + self.layers[i + 1].last().unwrap();
            self.layers[i].push(v);
        }

        *self.layers[0].last().unwrap()
    }
}

fn parse_numbers(input: &str) -> Vec<isize> {
    input
        .split_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .collect()
}

fn all_zero(list: &[isize]) -> bool {
    list.iter().all(|v| *v == 0)
}

fn part1(input: &str) -> isize {
    let mut histories = History::parse_list(input);
    histories.iter_mut().map(|h| h.extrapolate()).sum()
}

// fn part2(input: &str) -> isize {
//     let histories = Data::parse(input);
//     dbg!(&histories);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 114);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 2008960228);
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
