pub mod alu;
pub mod common;
pub mod solver;

fn main() {
    println!("part 1 result: {:?}", part1());
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn part1() -> usize {
    let instructions = common::alu_instructions();

    // reduce equation and solve
    let reduced = solver::reduce_model_number(&instructions);
    let values = reduced.maximum_input_for_z_value_of_zero();

    // double check on ALU
    let valid_on_alu =
        alu::validate_model_number(&values.iter().map(|v| *v as u8).collect(), &instructions);
    assert!(valid_on_alu);

    // convert into number
    values.iter().fold(0, |acc, v| acc * 10 + (*v as usize))
}

// fn part2(input: &str) -> usize {
//     let data = Data::parse(input);
//     println!("{:?}", data);
//     data.execute()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_solution() {
        let result = part1();
        assert_eq!(result, 99799212949967);
    }

    // #[test]
    // fn test_part2_example1() {
    //     let result = part2(EXAMPLE1);
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}
