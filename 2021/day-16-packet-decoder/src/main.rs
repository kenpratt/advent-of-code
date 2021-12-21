pub mod bitstream;
pub mod packet;

use bitstream::BitStream;
use packet::Packet;

use std::collections::VecDeque;
use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

fn decode(input: &str) -> Packet {
    let mut stream = BitStream::from_str(input);
    Packet::read(&mut stream)
}

fn sum_versions(packet: &Packet) -> usize {
    let mut to_visit = VecDeque::new();
    to_visit.push_back(packet);

    let mut sum = 0;
    while !to_visit.is_empty() {
        let curr = to_visit.pop_front().unwrap();
        match curr {
            Packet::Literal { version, value: _ } => {
                sum += version;
            }
            Packet::Operator {
                version,
                type_id: _,
                sub_packets,
            } => {
                sum += version;
                for p in sub_packets {
                    to_visit.push_back(p);
                }
            }
        }
    }
    sum
}

fn part1(input: &str) -> usize {
    let packet = decode(input);
    println!("{:?}", packet);
    sum_versions(&packet)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "D2FE28";
    static EXAMPLE2: &str = "38006F45291200";
    static EXAMPLE3: &str = "EE00D40C823060";
    static EXAMPLE4: &str = "8A004A801A8002F478";
    static EXAMPLE5: &str = "620080001611562C8802118E34";
    static EXAMPLE6: &str = "C0015000016115A2E0802F182340";
    static EXAMPLE7: &str = "A0016C880162017C3686B18A3D4780";

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1(EXAMPLE1), 6);
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(part1(EXAMPLE2), 9);
    }

    #[test]
    fn test_part1_example3() {
        assert_eq!(part1(EXAMPLE3), 14);
    }

    #[test]
    fn test_part1_example4() {
        assert_eq!(part1(EXAMPLE4), 16);
    }

    #[test]
    fn test_part1_example5() {
        assert_eq!(part1(EXAMPLE5), 12);
    }

    #[test]
    fn test_part1_example6() {
        assert_eq!(part1(EXAMPLE6), 23);
    }

    #[test]
    fn test_part1_example7() {
        assert_eq!(part1(EXAMPLE7), 31);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 889);
    }
}
