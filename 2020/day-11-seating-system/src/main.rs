use std::fs;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug, PartialEq)]
struct Grid {
    cells: Vec<Cell>,
    neighbour_map: Vec<Vec<usize>>,
    width: usize,
    height: usize,
    max_seats: usize,
}

impl Grid {
    fn parse(input: &str, view_cones: bool) -> Grid {
        let rows: Vec<Vec<Cell>> = input.lines().map(|line| Grid::parse_row(line)).collect();
        let width = rows.first().unwrap().len();
        let height = rows.len();

        let mut cells = vec![];
        for mut row in rows {
            cells.append(&mut row);
        }

        let neighbour_map = Grid::build_neighbour_map(&cells, width, height, view_cones);

        return Grid {
            cells: cells,
            neighbour_map: neighbour_map,
            width: width,
            height: height,
            max_seats: if view_cones {5} else {4},
        }
    }

    fn build_neighbour_map(cells: &Vec<Cell>, width: usize, height: usize, view_cones: bool) -> Vec<Vec<usize>> {
        return if view_cones {
            (0..cells.len()).map(|i| Grid::visual_neighbours(i, width, height, cells)).collect()
        } else {
            (0..cells.len()).map(|i| Grid::direct_neighbours(i, width, height)).collect()
        };
    }

    fn parse_row(input: &str) -> Vec<Cell> {
        return input.chars().map(|c| Cell::parse(&c)).collect();
    }

    fn iterate_until_stable(self) -> Grid {
        let mut last = self;
        let mut next = last.iterate();
        while next != last {
            last = next;
            next = last.iterate();
        }
        return next;
    }

    fn iterate(&self) -> Grid {
        let new_cells = self.cells.iter().enumerate().map(|(pos, c)| self.iterate_cell(pos, c)).collect();

        return Grid {
            cells: new_cells,
            neighbour_map: self.neighbour_map.clone(),
            width: self.width,
            height: self.height,            
            max_seats: self.max_seats,
        }
    }

    fn iterate_cell(&self, position: usize, cell: &Cell) -> Cell {
        // If a seat is empty (L) and there are no occupied seats adjacent to
        // it, the seat becomes occupied.
        // If a seat is occupied (#) and four/five or more seats adjacent to it are
        // also occupied, the seat becomes empty.
        // Otherwise, the seat's state does not change.
        return match cell {
            Cell::Floor => Cell::Floor,
            Cell::EmptySeat => {
                if self.num_occupied_neighbours(position) == 0 {
                    Cell::OccupiedSeat
                } else {
                    Cell::EmptySeat
                }
            },
            Cell::OccupiedSeat => {
                if self.num_occupied_neighbours(position) >= self.max_seats {
                    Cell::EmptySeat
                } else {
                    Cell::OccupiedSeat
                }
            },
        };
    }

    fn neighbour_in_direction(index: usize, width: usize, height: usize, direction: &Direction) -> Option<usize> {
        let row = index / width;
        let col = index % width;

        let right = width - 1;
        let bottom = height - 1;

        return match direction {
            Direction::UpLeft => if row > 0 && col > 0 {Some(index - width - 1)} else {None},
            Direction::Up => if row > 0 {Some(index - width)} else {None},
            Direction::UpRight => if row > 0 && col < right {Some(index - width + 1)} else {None},
            Direction::Left => if col > 0 {Some(index - 1)} else {None},
            Direction::Right => if col < right {Some(index + 1)} else {None},
            Direction::DownLeft => if row < bottom && col > 0 {Some(index + width - 1)} else {None},
            Direction::Down => if row < bottom {Some(index + width)} else {None},
            Direction::DownRight => if row < bottom && col < right {Some(index + width + 1)} else {None},
        };
    }

    fn direct_neighbours(position: usize, width: usize, height: usize) -> Vec<usize> {
        return DIRECTIONS.iter().map(|d| Grid::neighbour_in_direction(position, width, height, d)).filter(|o| o.is_some()).map(|o| o.unwrap()).collect();
    }

    fn visual_neighbours(position: usize, width: usize, height: usize, cells: &Vec<Cell>) -> Vec<usize> {
        return DIRECTIONS.iter().map(|d| Grid::first_seat_in_direction(position, width, height, cells, d)).filter(|o| o.is_some()).map(|o| o.unwrap()).collect();
    }

    fn first_seat_in_direction(position: usize, width: usize, height: usize, cells: &Vec<Cell>, direction: &Direction) -> Option<usize> {
        return match Grid::neighbour_in_direction(position, width, height, direction) {
            Some(neighbour) => {
                if cells[neighbour] == Cell::Floor {
                    Grid::first_seat_in_direction(neighbour, width, height, cells, direction)
                } else {
                    Some(neighbour)
                }
            },
            None => None,
        }
    }

    fn num_occupied_neighbours(&self, position: usize) -> usize {
        let neighbours = &self.neighbour_map[position];
        return neighbours.iter().filter(|i| self.cells[**i] == Cell::OccupiedSeat).count();
    }

    fn render(&self) -> String {
        let chars: Vec<char> = self.cells.iter().map(|c| c.render()).collect();
        let lines: Vec<String> = chars.chunks(self.width).map(|c| c.into_iter().collect()).collect();
        return lines.join("\n") + "\n";
    }

    fn num_occupied(&self) -> usize {
        return self.cells.iter().filter(|c| **c == Cell::OccupiedSeat).count();
    }
}

#[derive(Debug, PartialEq)]
enum Cell {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Cell {
    fn parse(input: &char) -> Cell {
        return match input {
            '.' => Cell::Floor,
            'L' => Cell::EmptySeat,
            '#' => Cell::OccupiedSeat,
            _ => panic!("Unknown cell value"),
        };
    }

    fn render(&self) -> char {
        return match self {
            Cell::Floor => '.',
            Cell::EmptySeat => 'L',
            Cell::OccupiedSeat => '#',
        }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    UpLeft,
    Up,
    UpRight,
    Left,
    Right,
    DownLeft,
    Down,
    DownRight,
}

static DIRECTIONS: &'static [Direction] = &[
    Direction::UpLeft,
    Direction::Up,
    Direction::UpRight,
    Direction::Left,
    Direction::Right,
    Direction::DownLeft,
    Direction::Down,
    Direction::DownRight,
];

fn part1(input: &str) -> usize {
    let initial_grid = Grid::parse(input, false);
    let final_grid = initial_grid.iterate_until_stable();
    return final_grid.num_occupied();
}

fn part2(input: &str) -> usize {
    let initial_grid = Grid::parse(input, true);
    let final_grid = initial_grid.iterate_until_stable();
    return final_grid.num_occupied();
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL
    "};

    static EXAMPLE1_ITERATION1: &str = indoc! {"
        #.##.##.##
        #######.##
        #.#.#..#..
        ####.##.##
        #.##.##.##
        #.#####.##
        ..#.#.....
        ##########
        #.######.#
        #.#####.##
    "};

    static EXAMPLE1_ITERATION2: &str = indoc! {"
        #.LL.L#.##
        #LLLLLL.L#
        L.L.L..L..
        #LLL.LL.L#
        #.LL.LL.LL
        #.LLLL#.##
        ..L.L.....
        #LLLLLLLL#
        #.LLLLLL.L
        #.#LLLL.##
    "};

    static EXAMPLE1_ITERATION3: &str = indoc! {"
        #.##.L#.##
        #L###LL.L#
        L.#.#..#..
        #L##.##.L#
        #.##.LL.LL
        #.###L#.##
        ..#.#.....
        #L######L#
        #.LL###L.L
        #.#L###.##
    "};

    static EXAMPLE1_ITERATION4: &str = indoc! {"
        #.#L.L#.##
        #LLL#LL.L#
        L.L.L..#..
        #LLL.##.L#
        #.LL.LL.LL
        #.LL#L#.##
        ..L.L.....
        #L#LLLL#L#
        #.LLLLLL.L
        #.#L#L#.##
    "};

    static EXAMPLE1_ITERATION5: &str = indoc! {"
        #.#L.L#.##
        #LLL#LL.L#
        L.#.L..#..
        #L##.##.L#
        #.#L.LL.LL
        #.#L#L#.##
        ..L.L.....
        #L#L##L#L#
        #.LLLLLL.L
        #.#L#L#.##
    "};

    #[test]
    fn test_example1_iterations() {
        let grid = Grid::parse(EXAMPLE1, false);

        let grid1 = grid.iterate();
        assert_eq!(grid1.render(), EXAMPLE1_ITERATION1);

        let grid2 = grid1.iterate();
        assert_eq!(grid2.render(), EXAMPLE1_ITERATION2);

        let grid3 = grid2.iterate();
        assert_eq!(grid3.render(), EXAMPLE1_ITERATION3);

        let grid4 = grid3.iterate();
        assert_eq!(grid4.render(), EXAMPLE1_ITERATION4);

        let grid5 = grid4.iterate();
        assert_eq!(grid5.render(), EXAMPLE1_ITERATION5);
    }

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 37);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 2316);
    }

    static EXAMPLE1_PART2_ITERATION1: &str = indoc! {"
        #.##.##.##
        #######.##
        #.#.#..#..
        ####.##.##
        #.##.##.##
        #.#####.##
        ..#.#.....
        ##########
        #.######.#
        #.#####.##
    "};

    static EXAMPLE1_PART2_ITERATION2: &str = indoc! {"
        #.LL.LL.L#
        #LLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLL#
        #.LLLLLL.L
        #.LLLLL.L#
    "};

    static EXAMPLE1_PART2_ITERATION3: &str = indoc! {"
        #.L#.##.L#
        #L#####.LL
        L.#.#..#..
        ##L#.##.##
        #.##.#L.##
        #.#####.#L
        ..#.#.....
        LLL####LL#
        #.L#####.L
        #.L####.L#
    "};

    static EXAMPLE1_PART2_ITERATION4: &str = indoc! {"
        #.L#.L#.L#
        #LLLLLL.LL
        L.L.L..#..
        ##LL.LL.L#
        L.LL.LL.L#
        #.LLLLL.LL
        ..L.L.....
        LLLLLLLLL#
        #.LLLLL#.L
        #.L#LL#.L#
    "};

    static EXAMPLE1_PART2_ITERATION5: &str = indoc! {"
        #.L#.L#.L#
        #LLLLLL.LL
        L.L.L..#..
        ##L#.#L.L#
        L.L#.#L.L#
        #.L####.LL
        ..#.#.....
        LLL###LLL#
        #.LLLLL#.L
        #.L#LL#.L#
    "};

    static EXAMPLE1_PART2_ITERATION6: &str = indoc! {"
        #.L#.L#.L#
        #LLLLLL.LL
        L.L.L..#..
        ##L#.#L.L#
        L.L#.LL.L#
        #.LLLL#.LL
        ..#.L.....
        LLL###LLL#
        #.LLLLL#.L
        #.L#LL#.L#
    "};

    #[test]
    fn test_example1_part2_iterations() {
        let grid = Grid::parse(EXAMPLE1, true);

        let grid1 = grid.iterate();
        assert_eq!(grid1.render(), EXAMPLE1_PART2_ITERATION1);

        let grid2 = grid1.iterate();
        assert_eq!(grid2.render(), EXAMPLE1_PART2_ITERATION2);

        let grid3 = grid2.iterate();
        assert_eq!(grid3.render(), EXAMPLE1_PART2_ITERATION3);

        let grid4 = grid3.iterate();
        assert_eq!(grid4.render(), EXAMPLE1_PART2_ITERATION4);

        let grid5 = grid4.iterate();
        assert_eq!(grid5.render(), EXAMPLE1_PART2_ITERATION5);

        let grid6 = grid5.iterate();
        assert_eq!(grid6.render(), EXAMPLE1_PART2_ITERATION6);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 2128);
    }
}