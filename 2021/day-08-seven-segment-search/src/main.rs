use std::collections::HashMap;
use std::fs;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"\A(.*) \| (.*)\z").unwrap();

    // Segment displays
    static ref SEGMENTS_TO_DIGIT: HashMap<&'static str, usize> = {
        let mut m = HashMap::new();
        m.insert("ijkmno", 0);
        m.insert("kn", 1);
        m.insert("iklmo", 2);
        m.insert("iklno", 3);
        m.insert("jkln", 4);
        m.insert("ijlno", 5);
        m.insert("ijlmno", 6);
        m.insert("ikn", 7);
        m.insert("ijklmno", 8);
        m.insert("ijklno", 9);
        m
    };
}

static VALID_WIRES: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

fn parse(input: &str) -> Vec<Entry> {
    input.lines().map(|line| Entry::parse(line)).collect()
}

#[derive(Debug)]
struct Pattern(String);

impl Pattern {
    fn parse(str: &str) -> Pattern {
        Pattern(str.chars().sorted().collect::<String>())
    }

    fn contains(&self, wire: &char) -> bool {
        self.0.chars().any(|c| &c == wire)
    }

    fn difference(&self, other: &Pattern) -> Pattern {
        let mut chars: Vec<char> = self.0.chars().collect();
        let other_chars: Vec<char> = other.0.chars().collect();
        chars.retain(|c| !other_chars.contains(c));
        Pattern(chars.iter().collect())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn single_element(&self) -> char {
        if self.0.len() != 1 {
            panic!("Expected single element");
        }
        self.0.chars().next().unwrap()
    }

    fn apply_key(&self, key: &HashMap<char, char>) -> String {
        self.0
            .chars()
            .map(|c| key.get(&c).unwrap())
            .sorted()
            .collect()
    }
}

fn segments_to_digit(segments: &str) -> usize {
    *SEGMENTS_TO_DIGIT.get(segments).unwrap()
}

#[derive(Debug)]
struct Entry {
    signals: Vec<Pattern>,
    outputs: Vec<Pattern>,
}

impl Entry {
    fn parse(input: &str) -> Entry {
        let captures = LINE_RE.captures(input).unwrap();
        let signals_str = captures.get(1).unwrap().as_str();
        let outputs_str = captures.get(2).unwrap().as_str();
        let signals = signals_str
            .split_whitespace()
            .map(|s| Pattern::parse(s))
            .collect();
        let outputs = outputs_str
            .split_whitespace()
            .map(|s| Pattern::parse(s))
            .collect();
        Entry {
            signals: signals,
            outputs: outputs,
        }
    }

    fn solve(&self) -> usize {
        let key = SegmentGroups::new(&self.signals).solve_key();
        println!("key: {:?}", key);

        let outputs: Vec<usize> = self
            .outputs
            .iter()
            .map(|o| Entry::read_value(o, &key))
            .collect();
        println!("outputs: {:?}", outputs);

        outputs.iter().fold(0, |acc, val| acc * 10 + val)
    }

    fn read_value(pattern: &Pattern, key: &HashMap<char, char>) -> usize {
        let segments = pattern.apply_key(key);
        println!("pattern {:?} => segments {:?}", pattern, segments);
        segments_to_digit(&segments)
    }
}

#[derive(Debug)]
struct SegmentGroups<'a> {
    one: &'a Pattern,
    four: &'a Pattern,
    seven: &'a Pattern,
    eight: &'a Pattern,
    two_three_five: Vec<&'a Pattern>,
    zero_six_nine: Vec<&'a Pattern>,
}

impl<'a> SegmentGroups<'a> {
    fn new(patterns: &[Pattern]) -> SegmentGroups {
        if patterns.len() != 10 {
            panic!("Expected 10 patterns");
        }
        SegmentGroups {
            one: SegmentGroups::find_single(patterns, 2),
            four: SegmentGroups::find_single(patterns, 4),
            seven: SegmentGroups::find_single(patterns, 3),
            eight: SegmentGroups::find_single(patterns, 7),
            two_three_five: SegmentGroups::find_triple(patterns, 5),
            zero_six_nine: SegmentGroups::find_triple(patterns, 6),
        }
    }

    fn find_single(patterns: &[Pattern], len: usize) -> &Pattern {
        let matches: Vec<&Pattern> = patterns.iter().filter(|p| p.len() == len).collect();
        if matches.len() != 1 {
            panic!("Bad input, expected 1 pattern of length {}", len);
        }
        matches[0]
    }

    fn find_triple(patterns: &[Pattern], len: usize) -> Vec<&Pattern> {
        let matches: Vec<&Pattern> = patterns.iter().filter(|p| p.len() == len).collect();
        if matches.len() != 3 {
            panic!("Bad input, expected 3 patterns of length {}", len);
        }
        matches
    }

    fn solve_key(&self) -> HashMap<char, char> {
        println!("{:?}", self);

        // use 1 & 7 to find segment I
        let kn = self.one;
        let ikn = self.seven;
        let i = ikn.difference(kn);
        println!("ikn:{:?} - kn:{:?} = i:{:?}", ikn, kn, i);

        // use 1 & 4 to find segments jl
        let jkln = self.four;
        let jl = jkln.difference(kn);
        println!("jkln:{:?} - kn:{:?} = jl:{:?}", jkln, kn, jl);

        // use 2 & 3 & 5 to find segments jm
        let jm = SegmentGroups::wires_with_count(&self.two_three_five, 1);
        println!(
            "frequency of 1 in 2/3/5:{:?} = jm:{:?}",
            self.two_three_five, jm
        );

        // now we can solve for l
        let l = jl.difference(&jm);
        println!("jl:{:?} - jm:{:?} = l:{:?}", jl, jm, l);

        // and j
        let j = jl.difference(&l);
        println!("jl:{:?} - l:{:?} = j:{:?}", jl, l, j);

        // and m
        let m = jm.difference(&j);
        println!("jm:{:?} - j:{:?} = m:{:?}", jm, j, m);

        // now we've solved values for i,j,l,m and the pair of kn
        // haven't tried o yet

        // use 2 & 3 & 5 to find sets of ilo and solve for o
        let ilo = SegmentGroups::wires_with_count(&self.two_three_five, 3);
        println!(
            "frequency of 3 in 2/3/5:{:?} = ilo:{:?}",
            self.two_three_five, ilo
        );
        let o = ilo.difference(&i).difference(&l);
        println!("ilo:{:?} - i:{:?} - l:{:?} = o:{:?}", ilo, i, l, o);

        // okay now we should be able to solve kn by distinguishing 2
        let iklmo = self
            .two_three_five
            .iter()
            .find(|p| p.contains(&m.single_element()))
            .unwrap();
        let k = iklmo
            .difference(&i)
            .difference(&l)
            .difference(&m)
            .difference(&o);
        println!(
            "iklmo:{:?} - i:{:?} - l:{:?} - m:{:?} - o:{:?} = k:{:?}",
            iklmo, i, l, m, o, k
        );

        // and solve for n
        let n = kn.difference(&k);
        println!("kn:{:?} - k:{:?} = n:{:?}", kn, k, n);

        let mut key = HashMap::new();
        key.insert(i.single_element(), 'i');
        key.insert(j.single_element(), 'j');
        key.insert(k.single_element(), 'k');
        key.insert(l.single_element(), 'l');
        key.insert(m.single_element(), 'm');
        key.insert(n.single_element(), 'n');
        key.insert(o.single_element(), 'o');
        key
    }

    fn wires_with_count(patterns: &[&Pattern], target: usize) -> Pattern {
        Pattern(
            VALID_WIRES
                .iter()
                .filter(|w| patterns.iter().filter(|p| p.contains(w)).count() == target)
                .collect(),
        )
    }
}

fn part1(input: &str) -> usize {
    let entries = parse(input);
    entries
        .iter()
        .map(|e| {
            e.outputs
                .iter()
                .filter(|o| o.len() < 5 || o.len() > 6)
                .count()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let entries = parse(input);
    entries.iter().map(|e| e.solve()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
    "};

    static EXAMPLE2: &str = indoc! {"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part1_example2() {
        let result = part1(EXAMPLE2);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 375);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 5353);
    }

    #[test]
    fn test_part2_example2() {
        let result = part2(EXAMPLE2);
        assert_eq!(result, 61229);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 1019355);
    }
}
