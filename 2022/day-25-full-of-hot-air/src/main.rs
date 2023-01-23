use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

fn snafu_to_decimal(num: &str) -> usize {
    num.chars()
        .fold(0, |acc, c| acc * 5 + snafu_char_to_int(&c)) as usize
}

fn decimal_to_snafu(num: usize) -> String {
    let mut rev_digits: Vec<char> = vec![];

    let mut val = num;
    while val > 0 {
        let mut div = val / 5;
        let mut rem = (val % 5) as isize;

        if rem > 2 {
            div += 1;
            rem -= 5;
        }

        val = div;
        rev_digits.push(int_to_snafu_char(rem));
    }

    rev_digits.into_iter().rev().collect()
}

fn snafu_char_to_int(c: &char) -> isize {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("Unexpected SNAFU char: {}", c),
    }
}

fn int_to_snafu_char(n: isize) -> char {
    match n {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => panic!("Unexpected int: {}", n),
    }
}

fn part1(input: &str) -> String {
    let decimal_nums: Vec<usize> = input
        .lines()
        .map(|s| {
            // verify
            let n = snafu_to_decimal(s);
            let s2 = decimal_to_snafu(n);
            assert_eq!(s, &s2);
            n
        })
        .collect();

    let sum = decimal_nums.iter().sum::<usize>();
    decimal_to_snafu(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        1=-0-2
        12111
        2=0=
        21
        2=01
        111
        20012
        112
        1=-1=
        1-12
        12
        1=
        122
    "};

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, "2=-1=0");
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, "2-121-=10=200==2==21");
    }
}
