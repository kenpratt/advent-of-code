use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

// for Enum iterator
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
    static ref SEGMENTS_FOR_0: HashSet<Segment> = [Segment::I, Segment::J, Segment::K, Segment::M, Segment::N, Segment::O].into();
    static ref SEGMENTS_FOR_1: HashSet<Segment> = [Segment::K, Segment::N].into();
    static ref SEGMENTS_FOR_2: HashSet<Segment> = [Segment::I, Segment::K, Segment::L, Segment::M, Segment::O].into();
    static ref SEGMENTS_FOR_3: HashSet<Segment> = [Segment::I, Segment::K, Segment::L, Segment::N, Segment::O].into();
    static ref SEGMENTS_FOR_4: HashSet<Segment> = [Segment::J, Segment::K, Segment::L, Segment::N].into();
    static ref SEGMENTS_FOR_5: HashSet<Segment> = [Segment::I, Segment::J, Segment::L, Segment::N, Segment::O].into();
    static ref SEGMENTS_FOR_6: HashSet<Segment> = [Segment::I, Segment::J, Segment::L, Segment::M, Segment::N, Segment::O].into();
    static ref SEGMENTS_FOR_7: HashSet<Segment> = [Segment::I, Segment::K, Segment::N].into();
    static ref SEGMENTS_FOR_8: HashSet<Segment> = [Segment::I, Segment::J, Segment::K, Segment::L, Segment::M, Segment::N, Segment::O].into();
    static ref SEGMENTS_FOR_9: HashSet<Segment> = [Segment::I, Segment::J, Segment::K, Segment::L, Segment::N, Segment::O].into();
}

fn parse(input: &str) -> Vec<Entry> {
    input.lines().map(|line| Entry::parse(line)).collect()
}

#[derive(Clone, Copy, Debug, EnumIter, Hash, Eq, PartialEq)]
enum Wire {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Wire {
    fn from_char(c: &char) -> Wire {
        match c {
            'a' => Wire::A,
            'b' => Wire::B,
            'c' => Wire::C,
            'd' => Wire::D,
            'e' => Wire::E,
            'f' => Wire::F,
            'g' => Wire::G,
            _ => panic!("Unexpected wire character: {}", c),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Pattern(HashSet<Wire>);

impl Pattern {
    fn parse(str: &str) -> Pattern {
        Pattern(str.chars().map(|c| Wire::from_char(&c)).collect())
    }

    fn contains(&self, wire: &Wire) -> bool {
        self.0.contains(wire)
    }

    fn difference(&self, other: &Pattern) -> Pattern {
        Pattern(self.0.difference(&other.0).cloned().collect())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn single_element(&self) -> Wire {
        if self.0.len() != 1 {
            panic!("Expected single element");
        }
        *self.0.iter().next().unwrap()
    }
}

// to avoid confusion, wires will be ABCDEFG, and segments will be IJKLMNO
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Segment {
    I,
    J,
    K,
    L,
    M,
    N,
    O,
}

fn segments_to_int(segments: &HashSet<Segment>) -> usize {
    if segments == &(*SEGMENTS_FOR_0) {
        0
    } else if segments == &(*SEGMENTS_FOR_1) {
        1
    } else if segments == &(*SEGMENTS_FOR_2) {
        2
    } else if segments == &(*SEGMENTS_FOR_3) {
        3
    } else if segments == &(*SEGMENTS_FOR_4) {
        4
    } else if segments == &(*SEGMENTS_FOR_5) {
        5
    } else if segments == &(*SEGMENTS_FOR_6) {
        6
    } else if segments == &(*SEGMENTS_FOR_7) {
        7
    } else if segments == &(*SEGMENTS_FOR_8) {
        8
    } else if segments == &(*SEGMENTS_FOR_9) {
        9
    } else {
        panic!("fooey")
    }
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
        let key = self.solve_key();
        println!("key: {:?}", key);

        let outputs: Vec<usize> = self
            .outputs
            .iter()
            .map(|o| Entry::read_value(o, &key))
            .collect();
        println!("outputs: {:?}", outputs);

        outputs.iter().fold(0, |acc, val| acc * 10 + val)
    }

    fn solve_key(&self) -> HashMap<Wire, Segment> {
        let signal_groups = SegmentGroups::new(&self.signals);
        println!("{:?}", signal_groups);

        // use 1 & 7 to find segment I
        let kn = signal_groups.one;
        let ikn = signal_groups.seven;
        let i = ikn.difference(kn);
        println!("ikn:{:?} - kn:{:?} = i:{:?}", ikn, kn, i);

        // use 1 & 4 to find segments jl
        let jkln = signal_groups.four;
        let jl = jkln.difference(kn);
        println!("jkln:{:?} - kn:{:?} = jl:{:?}", jkln, kn, jl);

        // use 2 & 3 & 5 to find segments jm
        let jm = Entry::wires_with_count(&signal_groups.two_three_five, 1);
        println!(
            "frequency of 1 in 2/3/5:{:?} = jm:{:?}",
            signal_groups.two_three_five, jm
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
        let ilo = Entry::wires_with_count(&signal_groups.two_three_five, 3);
        println!(
            "frequency of 3 in 2/3/5:{:?} = ilo:{:?}",
            signal_groups.two_three_five, ilo
        );
        let o = ilo.difference(&i).difference(&l);
        println!("ilo:{:?} - i:{:?} - l:{:?} = o:{:?}", ilo, i, l, o);

        // okay now we should be able to solve kn by distinguishing 2
        let iklmo = signal_groups
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
        key.insert(i.single_element(), Segment::I);
        key.insert(j.single_element(), Segment::J);
        key.insert(k.single_element(), Segment::K);
        key.insert(l.single_element(), Segment::L);
        key.insert(m.single_element(), Segment::M);
        key.insert(n.single_element(), Segment::N);
        key.insert(o.single_element(), Segment::O);
        key
    }

    fn wires_with_count(patterns: &[&Pattern], target: usize) -> Pattern {
        Pattern(
            Wire::iter()
                .filter(|w| patterns.iter().filter(|p| p.contains(w)).count() == target)
                .collect(),
        )
    }

    fn read_value(pattern: &Pattern, key: &HashMap<Wire, Segment>) -> usize {
        let segments: HashSet<Segment> = pattern
            .0
            .iter()
            .map(|w| key.get(w).unwrap())
            .cloned()
            .collect();
        println!("pattern {:?} => segments {:?}", pattern, segments);
        segments_to_int(&segments)
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
