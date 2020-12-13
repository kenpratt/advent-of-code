use std::fs;

// use lazy_static::lazy_static;
// use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug, PartialEq)]
struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let rows: Vec<Vec<Cell>> = input.lines().map(|line| Grid::parse_row(line)).collect();
        let width = rows.first().unwrap().len();
        let height = rows.len();

        let mut cells = vec![];
        for mut row in rows {
            cells.append(&mut row);
        }

        return Grid {
            cells: cells,
            width: width,
            height: height,
        }
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
            width: self.width,
            height: self.height,
        }
    }

    fn iterate_cell(&self, position: usize, cell: &Cell) -> Cell {
        // If a seat is empty (L) and there are no occupied seats adjacent to
        // it, the seat becomes occupied.
        // If a seat is occupied (#) and four or more seats adjacent to it are
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
                if self.num_occupied_neighbours(position) >= 4 {
                    Cell::EmptySeat
                } else {
                    Cell::OccupiedSeat
                }
            },
        };
    }

    fn neighbours(&self, position: usize) -> Vec<usize> {
        let mut neighbours: Vec<usize> = vec![];

        let row = position / self.width;
        let col = position % self.width;

        let r = self.width - 1;
    
        if row > 0 {
            let p = position - self.width;
            if col > 0 {neighbours.push(p - 1)}
            neighbours.push(p);
            if col < r {neighbours.push(p + 1)}
        }

        if col > 0 {neighbours.push(position - 1)}
        if col < r {neighbours.push(position + 1)}

        if row < (self.height - 1) {
            let p = position + self.width;
            if col > 0 {neighbours.push(p - 1)}
            neighbours.push(p);
            if col < r {neighbours.push(p + 1)}
        }

        return neighbours;
    }

    fn num_occupied_neighbours(&self, position: usize) -> usize {
        return self.neighbours(position).into_iter().filter(|i| self.cells[*i] == Cell::OccupiedSeat).count();
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

fn part1(input: &str) -> usize {
    let initial_grid = Grid::parse(input);
    let final_grid = initial_grid.iterate_until_stable();
    return final_grid.num_occupied();
}

// fn part2(input: &str) -> usize {
//     let data = Grid::parse(input);
//     return data.execute();
// }

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
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 37);
    }

    #[test]
    fn test_example1_iterations() {
        let grid = Grid::parse(EXAMPLE1);
        println!("{}", grid.render());

        let grid1 = grid.iterate();
        println!("{}", grid1.render());
        assert_eq!(grid1.render(), EXAMPLE1_ITERATION1);

        let grid2 = grid1.iterate();
        println!("{}", grid2.render());
        assert_eq!(grid2.render(), EXAMPLE1_ITERATION2);

        let grid3 = grid2.iterate();
        assert_eq!(grid3.render(), EXAMPLE1_ITERATION3);

        let grid4 = grid3.iterate();
        assert_eq!(grid4.render(), EXAMPLE1_ITERATION4);

        let grid5 = grid4.iterate();
        assert_eq!(grid5.render(), EXAMPLE1_ITERATION5);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 2316);
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