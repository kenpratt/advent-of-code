use std::fs;
use std::collections::HashMap;

fn main() {
    // read input & split into lines
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let lines = contents.lines();

    println!("part 1:\n{}", part1(&lines));
    println!("part 2:\n{}", part2(&lines).expect("Didn't return a string"));
}

fn part1(lines: &std::str::Lines) -> usize {
    let scores = lines.clone().map(|l| score_line(l));

    let count_with_2 = scores.clone().filter(|map| map.values().any(|x| *x == 2)).count();
    let count_with_3 = scores.clone().filter(|map| map.values().any(|x| *x == 3)).count();

    return count_with_2 * count_with_3;
}

fn score_line(line: &str) -> HashMap<char, u8> {
    let mut scores = HashMap::new();

    for ch in line.chars() {
        let count = scores.entry(ch).or_insert(0);
        *count += 1;
    }

    return scores;
}

fn part2(lines: &std::str::Lines) -> Option<String> {
    let foo: Vec<&str> = lines.clone().collect();
    let len = foo.len();

    let mut best: Option<String> = None;

    for x in 0..len {
        for y in (x+1)..len {
            // println!("x: {}, y: {}", x, y);
            let in_common = letters_in_common(foo[x], foo[y]);
            let res = match best {
                Some(curr) => {
                    if in_common.len() > curr.len() {
                        in_common
                    } else {
                        curr
                    }
                },
                None => in_common,
            };
            best = Some(res);
        }
    }

    return best;
}

fn letters_in_common(word1: &str, word2: &str) -> String {
    let chars1: Vec<char> = word1.chars().collect();
    let chars2: Vec<char> = word2.chars().collect();
    let len: usize = std::cmp::min(chars1.len(), chars2.len());

    return (0..len).filter(|&i| chars1[i] == chars2[i]).map(|i| chars1[i]).collect();
 }
