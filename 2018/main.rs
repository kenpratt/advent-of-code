use std::fs;

fn main() {
    // read input & split into lines
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let lines = contents.lines();

    let part1 = part1(&lines);
    println!("part 1: {:?}", part1);

    let part2 = part2(&lines);
    println!("part 2: {:?}", part2);
}

fn part1(lines: &std::str::Lines) -> usize {
    return 0;
}

fn part2(lines: &std::str::Lines) -> usize {
    return 0;
}
