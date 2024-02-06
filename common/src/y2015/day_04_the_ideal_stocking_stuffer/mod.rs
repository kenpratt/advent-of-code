use crate::parallel::*;

use md5::*;

const INPUT: &'static str = "bgvyzdsv";

// each parallel worker will grab this many jobs at the
// same time, to avoid contention on the work queue.
const BATCH_SIZE: usize = 500;

pub fn run() {
    println!("part 1 result: {:?}", part1(INPUT));
    println!("part 2 result: {:?}", part2(INPUT));
}

fn calculate_md5(mut context: Context, num: &usize) -> Digest {
    context.consume(num.to_string());
    context.compute()
}

fn md5_has_5_leading_zeroes(digest: &Digest) -> bool {
    // first 5 hex chars are 0
    // => first 20 bits are 0
    // => first 2.5 bytes are 0
    digest.0[0] == 0 && digest.0[1] == 0 && (digest.0[2] & 240) == 0
}

fn md5_has_6_leading_zeroes(digest: &Digest) -> bool {
    // first 6 hex chars are 0
    // => first 24 bits are 0
    // => first 3 bytes are 0
    digest.0[0] == 0 && digest.0[1] == 0 && digest.0[2] == 0
}

fn find_first_num(key: &str, condition: fn(&Digest) -> bool) -> usize {
    // build initial context with key once, instead of on every iteration
    let mut context = Context::new();
    context.consume(key);

    parallel_find(1..usize::MAX, BATCH_SIZE, |i| {
        let digest = calculate_md5(context.clone(), &i);
        if condition(&digest) {
            Some(i)
        } else {
            None
        }
    })
    .unwrap()
}

fn part1(input: &str) -> usize {
    find_first_num(input, md5_has_5_leading_zeroes)
}

fn part2(input: &str) -> usize {
    find_first_num(input, md5_has_6_leading_zeroes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(part1("abcdef"), 609043);
        assert_eq!(part1("pqrstuv"), 1048970);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(INPUT);
        assert_eq!(result, 254575);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(INPUT);
        assert_eq!(result, 1038736);
    }
}
