use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

fn parse(input: &str) -> Vec<&str> {
    input.trim().split(',').collect()
}

fn hash(input: &str) -> usize {
    input.chars().fold(0, |acc, c| hash_next_char(acc, &c))
}

fn hash_next_char(acc: usize, c: &char) -> usize {
    ((acc + *c as usize) * 17) % 256
}

fn part1(input: &str) -> usize {
    parse(input).into_iter().map(|s| hash(s)).sum()
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

    static EXAMPLE1: &str = indoc! {"
        HASH
    "};

    static EXAMPLE2: &str = indoc! {"
        rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 52);
    }

    #[test]
    fn test_part1_example2() {
        let result = part1(EXAMPLE2);
        assert_eq!(result, 1320);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 512283);
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
