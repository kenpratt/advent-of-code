use std::fs;

fn main() {
    // read input & split into lines
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    assert_eq!(lines.len(), 1);
    let s = lines.first().unwrap().to_string();

    let part1 = part1(s.clone());
    println!("part 1: {:?}", part1);

    let part2 = part2(s.clone());
    println!("part 2: {:?}", part2);
}

fn part1(s: String) -> usize {
    let mut done = false;
    let mut curr = s;
    while !done {
        let (next, num_reductions) = reduce(curr);
        curr = next;
        done = num_reductions == 0;
    }
    return curr.len();
}

fn reduce(s: String) -> (String, u32) {
    let mut iter = s.chars().peekable();

    let mut out = vec![];
    let mut reductions = 0;

    let mut done = false;
    while !done {
        match iter.next() {
            Some(curr) => {
                match iter.peek() {
                    Some(next) => {
                        let reduce = curr.is_ascii_lowercase() != next.is_ascii_lowercase() &&
                            curr.to_ascii_lowercase() == next.to_ascii_lowercase();

                        if reduce {
                            // skip next
                            iter.next();
                            reductions += 1;
                        } else {
                            // add curr to output, leaving next for the next iteration
                            out.push(curr);
                        }
                    },
                    None => {
                        // second to last char in the string, just append it
                        out.push(curr);
                    },
                }
            },
            None => {
                // end of the string
                done = true;
            },
        }
    }

    (out.into_iter().collect::<String>(), reductions)
}

fn part2(s: String) -> usize {
    let ascii_iter = (0..26).map(|x| (x + 'a' as u8) as char);

    return ascii_iter.map(|c| part1(strip(s.clone(), c))).min().unwrap();
}

fn strip(s: String, c: char) -> String {
    return s.chars().filter(|x| x.to_ascii_lowercase() != c).into_iter().collect::<String>();
}
