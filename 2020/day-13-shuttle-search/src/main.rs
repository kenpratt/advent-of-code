use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct Data {
    earliest_time: usize,
    bus_list: Vec<(usize, usize)>,
}

impl Data {
    fn parse(input: &str) -> Data {
        let lines: Vec<&str> = input.lines().collect();
        assert_eq!(lines.len(), 2);
        let earliest_time = lines[0].parse::<usize>().unwrap();
        let bus_list = Data::parse_bus_list(&lines[1]);

        return Data {
            earliest_time: earliest_time,
            bus_list: bus_list,
        }
    }

    fn parse_bus_list(input: &str) -> Vec<(usize, usize)> {
        let enumerated = input.split(',').enumerate();
        let valid = enumerated.filter(|(_, id)| id != &"x");
        let parsed = valid.map(|(offset, id)| (id.parse::<usize>().unwrap(), offset));
        return parsed.collect();
    }

    fn earliest_available_bus(&self) -> (usize, usize) {
        let bus_ids = self.bus_list.iter().map(|(id, _)| id);
        return bus_ids.map(|id| (*id, self.calculate_wait(id))).min_by_key(|p| p.1).unwrap();
    }

    fn calculate_wait(&self, bus_id: &usize) -> usize {
        return bus_id - self.earliest_time % bus_id;
    }
    
    fn earliest_timestamp_of_sequential_departures(&self) -> usize {
        println!("{:?}", self.bus_list);

        let (anchor_id, anchor_offset) = self.bus_list[0];
        assert_eq!(anchor_offset, 0);

        let mut base_t = 0;
        let mut base_id = anchor_id;

        for (id, offset) in &self.bus_list[1..] {
            base_t = Data::earlest_timestamp_for_busses_converging(base_t, base_id, *id, *offset);
            base_id = base_id * id;
        }

        return base_t;
    }

    fn earlest_timestamp_for_busses_converging(base_t: usize, base_id: usize, id: usize, offset: usize) -> usize {
        println!("base={}, chunk={}, id={}, offset={}", base_t, base_id, id, offset);
        for i in 0..id {
            let t = base_t + base_id * i;
            println!("  i={}, t={}, v={}, rem={}", i, t, t + offset, (t + offset) % id);
            if (t + offset) % id == 0 {
                return t;
            }
        }
        panic!("Should be unreachable");
    }
}

fn part1(input: &str) -> usize {
    let data = Data::parse(input);
    let (bus_id, wait) = data.earliest_available_bus();
    return bus_id * wait;
}

fn part2(input: &str) -> usize {
    let data = Data::parse(input);
    return data.earliest_timestamp_of_sequential_departures();
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        939
        7,13,x,x,59,x,31,19
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 295);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 205);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 1068781);
    }

    #[test]
    fn test_part2_example2() {
        let result = part2("0\n17,x,13,19");
        assert_eq!(result, 3417);
    }

    #[test]
    fn test_part2_example3() {
        let result = part2("0\n67,7,59,61");
        assert_eq!(result, 754018);
    }

    #[test]
    fn test_part2_example3x() {
        let result = part2("0\n67,x,59,61,x,x,x,x,7");
        assert_eq!(result, 754018);
    }

    #[test]
    fn test_part2_example4() {
        let result = part2("0\n67,x,7,59,61");
        assert_eq!(result, 779210);
    }

    #[test]
    fn test_part2_example5() {
        let result = part2("0\n67,7,x,59,61");
        assert_eq!(result, 1261476);
    }

    #[test]
    fn test_part2_example6() {
        let result = part2("0\n1789,37,47,1889");
        assert_eq!(result, 1202161486);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 803025030761664);
    }
}