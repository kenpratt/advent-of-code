pub mod alu;
pub mod common;
pub mod solver;

use crate::common::Goal;

fn main() {
    println!("part 1 result: {:?}", part1());
    println!("part 2 result: {:?}", part2());
}

fn solve_model_number(goal: &Goal) -> usize {
    let instructions = common::alu_instructions();

    // reduce equation and solve
    let reduced = solver::reduce_model_number(&instructions);
    let values = reduced.solve_inputs_for_z_of_0(goal);

    // double check on ALU
    let valid_on_alu =
        alu::validate_model_number(&values.iter().map(|v| *v as u8).collect(), &instructions);
    assert!(valid_on_alu);

    // convert into number
    values.iter().fold(0, |acc, v| acc * 10 + (*v as usize))
}

fn part1() -> usize {
    solve_model_number(&Goal::Maximum)
}

fn part2() -> usize {
    solve_model_number(&Goal::Minimum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_solution() {
        let result = part1();
        assert_eq!(result, 99799212949967);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2();
        assert_eq!(result, 34198111816311);
    }
}
