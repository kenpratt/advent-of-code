use std::fs;
use std::collections::HashSet;

fn main() {
    // read input & split into lines
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let lines = contents.lines();

    let numbers: Vec<i32> = lines.map(|s| s.parse::<i32>().expect("Couldn't parse string as an integer")).collect();

    println!("part 1:\n{}", find_sum(&numbers));
    println!("part 2:\n{}", find_first_repeat(&numbers));
}

fn find_sum(numbers: &[i32]) -> i32 {
    return numbers.into_iter().fold(0, |acc, x| acc + x);
}

fn find_first_repeat(numbers: &[i32]) -> i32 {
    let mut seen = HashSet::new();
    let mut curr = 0;

    loop {
        for n in numbers {
            curr += n;
            if seen.contains(&curr) {
                return curr;
            } else {
                seen.insert(curr.clone());
            }
        }
    }
}
