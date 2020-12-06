use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(read_input_file()));
    println!("part 2 result: {:?}", part2(read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

fn parse_input(input: &str) -> Vec<BoardingPass> {
    return input.lines().map(|line| BoardingPass::parse(line)).collect();
}

#[derive(Debug)]
struct BoardingPass {
    row: usize,
    column: usize,
    seat_id: usize,
}

impl BoardingPass {
    fn parse(input: &str) -> BoardingPass {
        if input.len() != 10 {
            panic!("Incorrect input string length");
        }

        println!("{:?}", input);

        let mut row_bsp = BinarySpacePartition::new(0, 128);
        for c in input[0..7].chars() {
            match c {
                'F' => row_bsp.take_left(),
                'B' => row_bsp.take_right(),
                _ => panic!("Unexpected character")
            }
            //println!("row char: {:?}, {:?}", c, row_bsp);
        }
        println!("row result: {:?}, {:?}", row_bsp, row_bsp.result());
        let row = row_bsp.result().unwrap();

        let mut column_bsp = BinarySpacePartition::new(0, 8);
        for c in input[7..10].chars() {
            match c {
                'L' => column_bsp.take_left(),
                'R' => column_bsp.take_right(),
                _ => panic!("Unexpected character")
            }
            //println!("column char: {:?}, {:?}", c, column_bsp);
        }
        println!("column result: {:?}, {:?}", column_bsp, column_bsp.result());
        let column = column_bsp.result().unwrap();

        let seat_id = row * 8 + column;

        return BoardingPass {
            row: row,
            column: column,
            seat_id: seat_id,
        };
    }
}

#[derive(Debug)]
struct BinarySpacePartition {
    offset: usize,
    size: usize,
}

impl BinarySpacePartition {
    fn new(offset: usize, size: usize) -> BinarySpacePartition {
        return BinarySpacePartition {
            offset: offset,
            size: size,
        }
    }

    fn take_left(&mut self) {
        self.size /= 2;
    }

    fn take_right(&mut self) {
        self.size /= 2;
        self.offset += self.size;
    }

    fn result(&self) -> Result<usize, &str> {
        if self.size == 1 {
            return Ok(self.offset);
        } else {
            return Err("Not done partitioning yet");
        }
    }
}

fn part1(input: String) -> usize {
    let passes = parse_input(&input);
    return passes.iter().map(|p| p.seat_id).max().unwrap();
}

fn part2(input: String) -> Result<usize, String> {
    let passes = parse_input(&input);

    let mut seat_ids: Vec<usize> = passes.iter().map(|p| p.seat_id).collect();
    seat_ids.sort();

    for i in 0..(seat_ids.len() - 2) {
        let seat1 = seat_ids[i];
        let seat2 = seat_ids[i+1];
        if seat2 - seat1 > 1 {
            return Ok(seat1 + 1);
        }
    }

    return Err("Didn't find a gap in the seats".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let pass = BoardingPass::parse("FBFBBFFRLR");
        assert_eq!(pass.row, 44);
        assert_eq!(pass.column, 5);
        assert_eq!(pass.seat_id, 357);
    }

    #[test]
    fn test_part1_example2() {
        let pass = BoardingPass::parse("BFFFBBFRRR");
        assert_eq!(pass.row, 70);
        assert_eq!(pass.column, 7);
        assert_eq!(pass.seat_id, 567);
    }

    #[test]
    fn test_part1_example3() {
        let pass = BoardingPass::parse("FFFBBBFRRR");
        assert_eq!(pass.row, 14);
        assert_eq!(pass.column, 7);
        assert_eq!(pass.seat_id, 119);
    }

    #[test]
    fn test_part1_example4() {
        let pass = BoardingPass::parse("BBFFBBFRLL");
        assert_eq!(pass.row, 102);
        assert_eq!(pass.column, 4);
        assert_eq!(pass.seat_id, 820);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(
            read_input_file()
        );
        assert_eq!(result, 953);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(
            read_input_file()
        );
        assert_eq!(result.unwrap(), 615);
    }
}