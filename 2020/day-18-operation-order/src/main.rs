pub mod lexer;
pub mod parser;

use std::fs;

use crate::parser::*;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct Calculations {
    list: Vec<Calculation>,
}

impl Calculations {
    fn parse(input: &str, use_operator_precedence: bool) -> Calculations {
        let list = input.lines().map(|line| Calculation::parse(line, use_operator_precedence)).collect();
        return Calculations {
            list: list,
        }
    }

    fn sum_of_values(&self) -> usize {
        return self.list.iter().map(|c| c.evaluate()).fold(0, |acc, x| acc + x);
    }
}

#[derive(Debug)]
struct Calculation {
    expression: Expression,
}

impl Calculation {
    fn parse(input: &str, use_operator_precedence: bool) -> Calculation {
        println!("input: {}", input);

        let tokens = lexer::tokenize(input).unwrap();
        println!("tokens: {:?}", tokens);

        let expression = parser::parse(&tokens, use_operator_precedence).unwrap();
        println!("ast: {:?}", expression);

        return Calculation {
            expression: expression,
        }
    }

    fn evaluate(&self) -> usize {
        Calculation::evaluate_expression(&self.expression) 
    }

    fn evaluate_expression(expression: &Expression) -> usize {
        match expression {
            Expression::Integer(value) => *value,
            Expression::Operation(operator, left, right) => {
                let left_val = Calculation::evaluate_expression(left);
                let right_val = Calculation::evaluate_expression(right);
                match operator {
                    '+' => left_val + right_val,
                    '*' => left_val * right_val,
                    _ => panic!("Unknown operator: {}", operator),
                }
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let data = Calculations::parse(input, false);
    return data.sum_of_values();
}

fn part2(input: &str) -> usize {
    let data = Calculations::parse(input, true);
    return data.sum_of_values();
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "1 + 2 * 3 + 4 * 5 + 6";
    static EXAMPLE2: &str = "1 + (2 * 3) + (4 * (5 + 6))";
    static EXAMPLE3: &str = "2 * 3 + (4 * 5)";
    static EXAMPLE4: &str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    static EXAMPLE5: &str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    static EXAMPLE6: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1(EXAMPLE1), 71);
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(part1(EXAMPLE2), 51);
    }

    #[test]
    fn test_part1_example3() {
        assert_eq!(part1(EXAMPLE3), 26);
    }

    #[test]
    fn test_part1_example4() {
        assert_eq!(part1(EXAMPLE4), 437);
    }

    #[test]
    fn test_part1_example5() {
        assert_eq!(part1(EXAMPLE5), 12240);
    }

    #[test]
    fn test_part1_example6() {
        assert_eq!(part1(EXAMPLE6), 13632);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 464478013511);
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(part2(EXAMPLE1), 231);
    }

    #[test]
    fn test_part2_example2() {
        assert_eq!(part2(EXAMPLE2), 51);
    }

    #[test]
    fn test_part2_example3() {
        assert_eq!(part2(EXAMPLE3), 46);
    }

    #[test]
    fn test_part2_example4() {
        assert_eq!(part2(EXAMPLE4), 1445);
    }

    #[test]
    fn test_part2_example5() {
        assert_eq!(part2(EXAMPLE5), 669060);
    }

    #[test]
    fn test_part2_example6() {
        assert_eq!(part2(EXAMPLE6), 23340);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 85660197232452);
    }
}