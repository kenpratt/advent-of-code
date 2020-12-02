use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input_str = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

fn run_part1(input: String) -> u64 {
    return 0;
}

fn part2() {
    let input_str = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1_example1() {
        let output = run_part1(
            "1721\n979\n366\n299\n675\n1456".to_string()
        );
        assert_eq!(output, 514579);
    }
}