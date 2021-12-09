use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

fn parse(input: &str) -> Vec<usize> {
    let line = input.lines().next().unwrap();
    line.split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn abs_diff(a: &usize, b: &usize) -> usize {
    if a >= b {
        a - b
    } else {
        b - a
    }
}

fn optimal_position_to_align_on(positions: &mut [usize]) -> usize {
    let min = positions.iter().min().unwrap();
    if *min != 0 {
        panic!("Currently expecting a minimum of 0");
    }

    positions.sort();

    let length = positions.len();
    let mid = length / 2;
    let median = positions[mid];

    let starting_position = median;
    let mut lowest_cost = align_cost(positions, &median);

    // try descending
    let mut current_position = starting_position - 1;
    loop {
        let current_cost = align_cost(positions, &current_position);
        if current_cost < lowest_cost {
            lowest_cost = current_cost;
            current_position -= 1;
        } else {
            // don't continue if our cost is static on increasing, it'll only get worse
            break;
        }
    }

    // try ascending
    let mut current_position = starting_position + 1;
    while current_position < length {
        let current_cost = align_cost(positions, &current_position);
        if current_cost < lowest_cost {
            lowest_cost = current_cost;
            current_position += 1;
        } else {
            // don't continue if our cost is static on increasing, it'll only get worse
            break;
        }
    }

    lowest_cost
}

fn align_cost(positions: &[usize], target: &usize) -> usize {
    let cost = positions.iter().map(|pos| abs_diff(pos, target)).sum();
    println!("cost({}) = {}", target, cost);
    cost
}

fn part1(input: &str) -> usize {
    let mut positions = parse(input);
    optimal_position_to_align_on(&mut positions)
}

// fn part2(input: &str) -> usize {
//     let data = parse(input);
//     println!("{:?}", data);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        16,1,2,0,4,2,7,1,2,14
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 37);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 345197);
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
