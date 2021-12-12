pub mod grid;

use grid::Cell;
use grid::Coordinate;
use grid::Grid;

use std::collections::VecDeque;
use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct Simulation {
    grid: Grid<u8>,
    flashes: usize,
}

impl Simulation {
    fn parse(input: &str) -> Simulation {
        let values: Vec<Vec<u8>> = input
            .lines()
            .map(|line| Simulation::parse_line(line))
            .collect();
        let grid = Grid::new(values);
        Simulation {
            grid: grid,
            flashes: 0,
        }
    }

    fn parse_line(line: &str) -> Vec<u8> {
        line.chars()
            .filter_map(|c| c.to_digit(10))
            .map(|v| v as u8)
            .collect()
    }

    fn tick(&mut self) {
        let mut to_flash = VecDeque::new();

        for cell in self.grid.iter_mut() {
            Simulation::increment_cell(cell, true, &mut to_flash);
        }

        loop {
            match to_flash.pop_front() {
                Some(pos) => {
                    self.flashes += 1;
                    for neighbour_pos in self.grid.neighbours(&pos) {
                        let neighbour = self.grid.cell_mut(&neighbour_pos);
                        Simulation::increment_cell(neighbour, false, &mut to_flash);
                    }
                }
                None => break,
            }
        }
    }

    fn increment_cell(cell: &mut Cell<u8>, first_pass: bool, to_flash: &mut VecDeque<Coordinate>) {
        match cell.value {
            0 => {
                if first_pass {
                    cell.value += 1;
                }
                // if not first_pass, this cell was already flashed/marked to be flashed, do nothing
            }
            9 => {
                cell.value = 0;
                to_flash.push_back(cell.position);
            }
            _ => cell.value += 1,
        }
    }

    fn render(&self) -> String {
        format!(
            "{}\nflashes: {}",
            self.grid.render(|v| v.to_string()),
            self.flashes
        )
    }
}

fn part1(input: &str) -> usize {
    let mut simulation = Simulation::parse(input);
    println!("{}\n", simulation.render());
    for i in 1..=100 {
        simulation.tick();
        println!("after round {}:\n{}\n", i, simulation.render());
    }
    simulation.flashes
}

fn part2(input: &str) -> usize {
    let mut simulation = Simulation::parse(input);
    println!("{}\n", simulation.render());
    let num_cells = simulation.grid.len();
    for i in 1..1000000 {
        let flashes_before = simulation.flashes;
        simulation.tick();
        println!("after round {}:\n{}\n", i, simulation.render());
        if (simulation.flashes - flashes_before) == num_cells {
            return i;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 1656);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 1743);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 195);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 364);
    }
}
