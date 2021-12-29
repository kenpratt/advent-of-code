use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Write;
use std::fs;
use std::rc::Rc;
use std::str::Chars;

use itertools::Itertools;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

fn parse(input: &str) -> Vec<ElementRef> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(input: &str) -> ElementRef {
    Parser::parse(input)
}

#[derive(Debug)]
struct Parser<'a> {
    chars: Chars<'a>,
}

impl<'a> Parser<'a> {
    fn parse(input: &'a str) -> ElementRef {
        let mut parser = Self {
            chars: input.chars(),
        };
        parser.parse_element().into()
    }

    fn parse_element(&mut self) -> Element {
        let c = self.chars.next().unwrap();
        match c {
            '[' => {
                // recurse on left, take comma, recurse on right, take ']'
                let left = self.parse_element();
                assert_eq!(self.chars.next().unwrap(), ',');
                let right = self.parse_element();
                assert_eq!(self.chars.next().unwrap(), ']');
                Element::Pair(left.into(), right.into())
            }
            d if d.is_digit(10) => Element::Value(d.to_digit(10).unwrap()),
            _ => panic!("Unexpected character during element parsing: {}", c),
        }
    }
}

#[derive(Debug)]
enum Element {
    Value(u32),
    Pair(ElementRef, ElementRef),
}

impl Element {
    fn deep_clone(&self) -> Element {
        match self {
            Element::Value(val) => Element::Value(*val),
            Element::Pair(left, right) => Element::Pair(left.deep_clone(), right.deep_clone()),
        }
    }

    fn value(&self) -> Option<u32> {
        match self {
            Element::Value(val) => Some(*val),
            Element::Pair(_, _) => None,
        }
    }

    fn is_value(&self) -> bool {
        match self {
            Element::Value(_) => true,
            Element::Pair(_, _) => false,
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Element::Value(val) => *val,
            Element::Pair(left, right) => {
                let lm = left.magnitude();
                let rm = right.magnitude();
                lm * 3 + rm * 2
            }
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Value(val) => write!(f, "{}", val),
            Self::Pair(left, right) => write!(f, "[{},{}]", left, right),
        }
    }
}

#[derive(Debug)]
struct ElementRef(Rc<RefCell<Element>>);

impl ElementRef {
    fn wrap(element: Element) -> Self {
        Self(Rc::new(RefCell::new(element)))
    }

    fn deep_clone(&self) -> Self {
        Self::wrap(self.0.borrow().deep_clone())
    }

    fn value(&self) -> Option<u32> {
        self.0.borrow().value()
    }

    fn is_value(&self) -> bool {
        self.0.borrow().is_value()
    }

    fn increase_value(&self, amount: u32) {
        match &mut *self.0.borrow_mut() {
            Element::Value(v) => *v += amount,
            _ => panic!("trying to increase a non-value element"),
        };
    }

    fn magnitude(&self) -> u32 {
        self.0.borrow().magnitude()
    }

    fn reduce(&self) {
        while self.reduce_once() {}
    }

    fn reduce_once(&self) -> bool {
        self.attempt_explode() || self.attempt_split()
    }

    fn attempt_explode(&self) -> bool {
        let mut last_value: Option<ElementRef> = None;
        let mut add_to_next_value = 0;
        let mut blew_up = false;
        for (elem, depth) in self {
            if !blew_up {
                // looking for something to blow up
                let mut to_blow_up = None;
                match &*elem.0.borrow() {
                    Element::Value(_) => {
                        last_value = Some(elem.clone());
                    }
                    Element::Pair(left, right) => {
                        if depth >= 4 {
                            let left_val: u32 = left.value().unwrap();
                            let right_val: u32 = right.value().unwrap();
                            to_blow_up = Some((elem.clone(), left_val, right_val));
                        }
                    }
                };

                // found a target, blow 'em up!
                if to_blow_up.is_some() {
                    let (target, left_val, right_val) = to_blow_up.take().unwrap();
                    println!("blowing up {}", target);

                    // replace current pair with 0
                    target.0.replace(Element::Value(0));

                    // increase last seen Value
                    last_value.take().map(|v| v.increase_value(left_val));

                    // store amount to add to next seen Value
                    add_to_next_value = right_val;

                    blew_up = true;
                }
            } else {
                // already blew up
                if elem.is_value() {
                    elem.increase_value(add_to_next_value);

                    // finished!
                    return blew_up;
                }
            }
        }
        blew_up
    }

    fn attempt_split(&self) -> bool {
        for (elem, _depth) in self {
            // looking for something to split
            let mut to_split = None;
            match &*elem.0.borrow() {
                Element::Value(v) => {
                    if *v >= 10 {
                        to_split = Some(elem.clone());
                    }
                }
                Element::Pair(_, _) => {}
            };

            // found a target, split it!
            if to_split.is_some() {
                let target = to_split.take().unwrap();
                let target_val = target.value().unwrap();
                println!("splitting {}", target);

                // left is v/2 rounded down, right is v/2 rounded up
                let left_val = target_val / 2;
                let right_val = target_val - left_val;

                // replace current pair with 0
                target.0.replace(
                    Element::Pair(
                        Element::Value(left_val).into(),
                        Element::Value(right_val).into(),
                    )
                    .into(),
                );

                // finished!
                return true;
            }
        }
        false
    }

    fn add(self, other: ElementRef) -> ElementRef {
        Element::Pair(self, other).into()
    }

    fn sum(self, other: ElementRef) -> ElementRef {
        let combined = self.add(other);
        combined.reduce();
        combined
    }
}

impl Clone for ElementRef {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl From<Element> for ElementRef {
    fn from(element: Element) -> Self {
        Self::wrap(element)
    }
}

impl fmt::Display for ElementRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.borrow())
    }
}

struct ElementRefIterator {
    to_visit: VecDeque<ElementRefIteratorItem>,
    previous_element: Option<ElementRefIteratorItem>,
}

#[derive(Clone, Debug)]
struct ElementRefIteratorItem(ElementRef, u8);

impl From<(ElementRef, u8)> for ElementRefIteratorItem {
    fn from(tuple: (ElementRef, u8)) -> Self {
        Self(tuple.0, tuple.1)
    }
}

impl From<ElementRefIteratorItem> for (ElementRef, u8) {
    fn from(item: ElementRefIteratorItem) -> Self {
        (item.0, item.1)
    }
}

impl fmt::Display for ElementRefIteratorItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} depth={}", self.0, self.1)
    }
}

impl ElementRefIterator {
    fn new(root: ElementRef) -> ElementRefIterator {
        let mut to_visit = VecDeque::new();
        to_visit.push_back((root, 0).into());
        ElementRefIterator {
            to_visit: to_visit,
            previous_element: None,
        }
    }

    fn expand_previous_element(&mut self) {
        if self.previous_element.is_some() {
            let (element, depth) = self.previous_element.take().unwrap().into();
            match &*element.0.borrow() {
                Element::Value(_) => {}
                Element::Pair(left, right) => {
                    self.to_visit.push_front((right.clone(), depth + 1).into());
                    self.to_visit.push_front((left.clone(), depth + 1).into());
                }
            };
        }
    }
}

impl Iterator for ElementRefIterator {
    type Item = (ElementRef, u8);

    fn next(&mut self) -> Option<Self::Item> {
        self.expand_previous_element();
        let elem = self.to_visit.pop_front();
        self.previous_element = elem.clone();
        elem.map(|e| e.into())
    }
}

impl IntoIterator for &ElementRef {
    type Item = (ElementRef, u8);
    type IntoIter = ElementRefIterator;

    fn into_iter(self) -> Self::IntoIter {
        ElementRefIterator::new(self.clone())
    }
}

fn sum(elements: Vec<ElementRef>) -> ElementRef {
    elements
        .into_iter()
        .reduce(|acc, elem| acc.sum(elem))
        .unwrap()
}

fn part1(input: &str) -> u32 {
    let pairs = parse(input);
    println!("{:?}", pairs);
    let result = sum(pairs);
    println!("sum: {:?}", result);
    result.magnitude()
}

fn pretty_pairs(pairs: &[ElementRef]) -> String {
    let mut s = String::new();
    for p in pairs {
        write!(&mut s, "\n  {}", p).unwrap();
    }
    s
}

fn part2(input: &str) -> u32 {
    let pairs = parse(input);
    println!("pairs: {}", pretty_pairs(&pairs));

    pairs
        .into_iter()
        .permutations(2)
        .map(|perm| {
            let cloned: Vec<ElementRef> = perm.iter().map(|e| e.deep_clone()).collect();
            println!("perm:{}", pretty_pairs(&cloned));
            let foo = sum(cloned).magnitude();
            println!("=> {:?}", foo);
            foo
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        [[[[4,3],4],4],[7,[[8,4],9]]]
        [1,1]
    "};
    static EXAMPLE1_SUM: &str = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";

    static EXAMPLE2: &str = indoc! {"
        [1,1]
        [2,2]
        [3,3]
        [4,4]
    "};
    static EXAMPLE2_SUM: &str = "[[[[1,1],[2,2]],[3,3]],[4,4]]";

    static EXAMPLE3: &str = indoc! {"
        [1,1]
        [2,2]
        [3,3]
        [4,4]
        [5,5]
    "};
    static EXAMPLE3_SUM: &str = "[[[[3,0],[5,3]],[4,4]],[5,5]]";

    static EXAMPLE4: &str = indoc! {"
        [1,1]
        [2,2]
        [3,3]
        [4,4]
        [5,5]
        [6,6]
    "};
    static EXAMPLE4_SUM: &str = "[[[[5,0],[7,4]],[5,5]],[6,6]]";

    static EXAMPLE5: &str = indoc! {"
        [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
        [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
        [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
        [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
        [7,[5,[[3,8],[1,4]]]]
        [[2,[2,2]],[8,[8,1]]]
        [2,9]
        [1,[[[9,3],9],[[9,0],[0,7]]]]
        [[[5,[7,4]],7],1]
        [[[[4,2],2],6],[8,7]]
    "};
    static EXAMPLE5_SUM: &str = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";

    static EXAMPLE6: &str = indoc! {"
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    "};
    static EXAMPLE6_SUM: &str = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]";
    static EXAMPLE6_MAGNITUDE: u32 = 4140;

    static MAGNITUDE_EXAMPLES: [(&str, u32); 9] = [
        ("[9,1]", 29),
        ("[1,9]", 21),
        ("[[9,1],[1,9]]", 129),
        ("[[1,2],[[3,4],5]]", 143),
        ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
        ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
        ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
        ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
        (
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            3488,
        ),
    ];

    #[test]
    fn test_sum() {
        assert_eq!(sum(parse(EXAMPLE1)).to_string(), EXAMPLE1_SUM);
        assert_eq!(sum(parse(EXAMPLE2)).to_string(), EXAMPLE2_SUM);
        assert_eq!(sum(parse(EXAMPLE3)).to_string(), EXAMPLE3_SUM);
        assert_eq!(sum(parse(EXAMPLE4)).to_string(), EXAMPLE4_SUM);
        assert_eq!(sum(parse(EXAMPLE5)).to_string(), EXAMPLE5_SUM);
        assert_eq!(sum(parse(EXAMPLE6)).to_string(), EXAMPLE6_SUM);
    }

    #[test]
    fn test_magnitude() {
        for (input, expected_result) in MAGNITUDE_EXAMPLES {
            assert_eq!(parse_line(input).magnitude(), expected_result);
        }
    }

    #[test]
    fn test_part1_example6() {
        assert_eq!(part1(EXAMPLE6), EXAMPLE6_MAGNITUDE);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 2541);
    }

    #[test]
    fn test_part2_example6() {
        assert_eq!(part2(EXAMPLE6), 3993);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 4647);
    }
}
