use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(read_input_file()));
    println!("part 2 result: {:?}", part2(read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
}

impl Map {
    fn parse(input: String) -> Map {
        let cells: Vec<Vec<bool>> = input.lines().map(|line| Map::parse_line(line)).collect();
        return Map {
            width: cells[0].len(),
            height: cells.len(),
            cells: cells,
        }
    }

    fn parse_line(line: &str) -> Vec<bool> {
        return line.chars().map(|c| Map::parse_char(&c)).collect();
    }

    fn parse_char(c: &char) -> bool {
        return match c {
            '.' => false,
            '#' => true,
            _ => panic!("Bad input on map")
        };
    }

    fn check_slope(&self, right: usize, down: usize) -> usize {
        let mut x = 0;
        let mut y = 0;
        let mut trees = 0;

        while y < self.height {
            if self.value(x, y) {
                trees += 1
            }

            x += right;
            y += down;
        }

        return trees;
    }

    fn value(&self, x: usize, y: usize) -> bool {
        return self.cells[y][x % self.width];
    }
}

fn part1(input: String) -> usize {
    let map = Map::parse(input);
    return map.check_slope(3, 1);
}

fn part2(input: String) -> usize {
    let map = Map::parse(input);

    let slopes = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    return slopes.iter().map(|(r, d)| map.check_slope(*r, *d)).fold(1, |acc, x| acc * x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = part1(
            "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#".to_string()
        );
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(
            read_input_file()
        );
        assert_eq!(result, 294);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(
            "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#".to_string()
        );
        assert_eq!(result, 336);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(
            read_input_file()
        );
        assert_eq!(result, 5774564250);
    }
}