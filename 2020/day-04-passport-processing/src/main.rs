use std::collections::HashMap;
use std::fs;

use indoc::indoc;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(read_input_file()));
    println!("part 2 result: {:?}", part2(read_input_file()));
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

    fn num_with_required_fields_present(&self) -> usize {
        return self.0.iter().filter(|p| p.required_fields_present()).count();
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

    fn required_fields_present(&self) -> bool {
        return REQUIRED_FIELDS.iter().all(|&key| self.0.contains_key(key));
    }

    fn valid(&self) -> bool {
        return REQUIRED_FIELDS.iter().all(|&key| self.validate_field(key));
    }

    fn validate_field(&self, key: &str) -> bool {
        return match self.0.get(key) {
            Some(val) => Passport::validate_field_value(key, val),
            None => false
        }
    }

    fn validate_field_value(key: &str, val: &str) -> bool {
        match key {
            // byr (Birth Year) - four digits; at least 1920 and at most 2002.
            "byr" => {
                lazy_static! {
                    static ref BYR_REGEX: Regex = Regex::new(r"^\d{4}$").unwrap();
                }
                if BYR_REGEX.is_match(val) {
                    let year = val.parse::<usize>().unwrap();
                    return year >= 1920 && year <= 2002;
                } else {
                    return false;
                }
            },

            // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            "iyr" => {
                lazy_static! {
                    static ref IYR_REGEX: Regex = Regex::new(r"^\d{4}$").unwrap();
                }
                if IYR_REGEX.is_match(val) {
                    let year = val.parse::<usize>().unwrap();
                    return year >= 2010 && year <= 2020;
                } else {
                    return false;
                }
            },

            // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
            "eyr" => {
                lazy_static! {
                    static ref EYR_REGEX: Regex = Regex::new(r"^\d{4}$").unwrap();
                }
                if EYR_REGEX.is_match(val) {
                    let year = val.parse::<usize>().unwrap();
                    return year >= 2020 && year <= 2030;
                } else {
                    return false;
                }
            },

            // hgt (Height) - a number followed by either cm or in:
            // If cm, the number must be at least 150 and at most 193.
            // If in, the number must be at least 59 and at most 76.
            "hgt" => {
                lazy_static! {
                    static ref HGT_REGEX: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
                }
                if HGT_REGEX.is_match(val) {
                    let captures = HGT_REGEX.captures(val).unwrap();
                    let height = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    let units = captures.get(2).unwrap().as_str();
                    match units {
                        "cm" => (height >= 150 && height <= 193),
                        "in" => (height >= 59 && height <= 76),
                        _ => panic!("Unreachable")
                    }
                } else {
                    return false;
                }
            },

            // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            "hcl" => {
                lazy_static! {
                    static ref HCL_REGEX: Regex = Regex::new(r"^\#[0-9a-f]{6}$").unwrap();
                }
                return HCL_REGEX.is_match(val);
            },

            // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            "ecl" => {
                lazy_static! {
                    static ref ECL_REGEX: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
                }
                return ECL_REGEX.is_match(val);
            },

            // pid (Passport ID) - a nine-digit number, including leading zeroes.
            "pid" => {
                lazy_static! {
                    static ref PID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
                }
                return PID_REGEX.is_match(val);
            },

            // cid (Country ID) - ignored, missing or not.
            "cid" => true, 

            // unknown field
            _ => panic!("Unknown field")
        }
    }
}

fn part1(input: String) -> usize {
    let passports = PassportList::parse(input);
    return passports.num_with_required_fields_present();
}

fn part2(input: String) -> usize {
    let passports = PassportList::parse(input);
    return passports.num_valid();
}

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

    #[test]
    fn test_field_validaton() {
        assert_eq!(Passport::validate_field_value("byr", "2002"), true);
        assert_eq!(Passport::validate_field_value("byr", "2003"), false);

        assert_eq!(Passport::validate_field_value("hgt", "60in"), true);
        assert_eq!(Passport::validate_field_value("hgt", "190cm"), true);
        assert_eq!(Passport::validate_field_value("hgt", "190in"), false);
        assert_eq!(Passport::validate_field_value("hgt", "190"), false);

        assert_eq!(Passport::validate_field_value("hcl", "#123abc"), true);
        assert_eq!(Passport::validate_field_value("hcl", "#123abz"), false);
        assert_eq!(Passport::validate_field_value("hcl", "123abc"), false);

        assert_eq!(Passport::validate_field_value("ecl", "brn"), true);
        assert_eq!(Passport::validate_field_value("ecl", "wat"), false);

        assert_eq!(Passport::validate_field_value("pid", "000000001"), true);
        assert_eq!(Passport::validate_field_value("pid", "0123456789"), false);
    }

    #[test]
    fn test_part2_example1() {
        let input = indoc! {"
            eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
            
            iyr:2019
            hcl:#602927 eyr:1967 hgt:170cm
            ecl:grn pid:012533040 byr:1946
            
            hcl:dab227 iyr:2012
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
            
            hgt:59cm ecl:zzz
            eyr:2038 hcl:74454a iyr:2023
            pid:3556412378 byr:2007
        "};
        let result = part2(input.to_string());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_example2() {
        let input = indoc! {"
            pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
            hcl:#623a2f
            
            eyr:2029 ecl:blu cid:129 byr:1989
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
            
            hcl:#888785
            hgt:164cm byr:2001 iyr:2015 cid:88
            pid:545766238 ecl:hzl
            eyr:2022
            
            iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        "};
        let result = part2(input.to_string());
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(
            read_input_file()
        );
        assert_eq!(result, 103);
    }
}