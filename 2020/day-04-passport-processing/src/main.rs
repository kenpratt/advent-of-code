use std::collections::HashMap;
use std::fs;

use indoc::indoc;

fn main() {
    println!("part 1 result: {:?}", part1(read_input_file()));
    //println!("part 2 result: {:?}", part2(read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

struct PassportList(Vec<Passport>);
struct Passport(HashMap<String, String>);

impl PassportList {
    fn parse(input: String) -> PassportList {
        let passports: Vec<Passport> = input.split("\n\n").map(|chunk| Passport::parse(chunk)).collect();
        return PassportList(passports);
    }

    fn num_valid(&self) -> usize {
        return self.0.iter().filter(|p| p.valid()).count();
    }
}

const REQUIRED_FIELDS: &'static [&'static str] = &[
    "byr", // (Birth Year)
    "iyr", // (Issue Year)
    "eyr", // (Expiration Year)
    "hgt", // (Height)
    "hcl", // (Hair Color)
    "ecl", // (Eye Color)
    "pid", // (Passport ID)
    //"cid", // (Country ID) (optional)
];

impl Passport {
    fn parse(input: &str) -> Passport {
        let mut map = HashMap::new();
        for part in input.split_whitespace() {
            let (key, val) = Passport::parse_field(part);
            map.insert(key, val);
        }
        return Passport(map);
    }

    fn parse_field(field: &str) -> (String, String) {
        let parts: Vec<&str> = field.split(':').collect();
        if parts.len() != 2 {
            panic!("Malformed input");
        }
        return (parts[0].to_string(), parts[1].to_string());
    }

    fn valid(&self) -> bool {
        return REQUIRED_FIELDS.iter().all(|&key| self.0.contains_key(key));
    }
}

fn part1(input: String) -> usize {
    let passports = PassportList::parse(input);
    return passports.num_valid();
}

// fn part2(input: String) -> usize {
//     let data = Data::parse(input);
//     return data.execute();
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = indoc! {"
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm
            
            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929

            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm

            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in
        "};
        let result = part1(input.to_string());
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(
            read_input_file()
        );
        assert_eq!(result, 170);
    }

    // #[test]
    // fn test_part2_example1() {
    //     let result = part2(
    //         "".to_string()
    //     );
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(
    //         read_input_file()
    //     );
    //     assert_eq!(result, 0);
    // }
}