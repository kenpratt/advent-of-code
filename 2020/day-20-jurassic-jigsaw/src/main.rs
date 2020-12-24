pub mod solver;
pub mod tile;

use std::collections::HashMap;
use std::fs;

use crate::tile::*;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    return fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}

#[derive(Debug)]
struct CameraArray {
    tiles: Vec<Tile>,
    array_width: usize,
}

impl CameraArray {
    fn parse(input: &str) -> CameraArray {
        let tiles: Vec<Tile> = input.split("\n\n").map(|chunk| Tile::parse(chunk)).collect();

        // assert all tiles are the same size, and square
        assert!(tiles[1..].iter().all(|t| t.width == tiles[0].width));
        assert!(tiles[1..].iter().all(|t| t.height == tiles[0].height));
        assert_eq!(tiles[0].width, tiles[0].height);

        // figure out array width
        let array_width = (tiles.len() as f64).sqrt().round() as usize;
        assert_eq!(array_width * array_width, tiles.len());

        return CameraArray {
            tiles: tiles,
            array_width: array_width,
        }
    }

    fn solve_tile_layout(&self) -> Vec<Vec<(usize, Direction, usize)>> {
        solver::solve(&self.tiles, self.array_width)
    }

    fn solve_for_corner_ids(&self) -> Vec<usize> {
        let layout = self.solve_tile_layout();
        let max = self.array_width - 1;
        vec![
            layout[0][0].0,
            layout[0][max].0,
            layout[max][0].0,
            layout[max][max].0,
        ]
    }

    fn solve_for_combined_image(&self) -> Tile {
        let tile_layout = self.solve_tile_layout();

        // swap tile objects for tile IDs
        let tile_lookup: HashMap<usize, &Tile> = self.tiles.iter().map(|t| (t.id, t)).collect();
        let tile_layout2: Vec<Vec<(&Tile, Direction, usize)>> = tile_layout.iter().map(|row| {
            row.iter().map(|(id, direction, rotation)| {
                let tile = tile_lookup.get(id).unwrap();
                (*tile, *direction, *rotation)
            }).collect()
        }).collect();

        Tile::merge(&tile_layout2)
    }
}

fn part1(input: &str) -> usize {
    let array = CameraArray::parse(input);
    array.solve_for_corner_ids().iter().fold(1, |acc, id| acc * id)
}

fn part2(input: &str) -> usize {
    let array = CameraArray::parse(input);
    let image = array.solve_for_combined_image();
    println!("{}", image.to_string());
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        Tile 2311:
        ..##.#..#.
        ##..#.....
        #...##..#.
        ####.#...#
        ##.##.###.
        ##...#.###
        .#.#.#..##
        ..#....#..
        ###...#.#.
        ..###..###
    
        Tile 1951:
        #.##...##.
        #.####...#
        .....#..##
        #...######
        .##.#....#
        .###.#####
        ###.##.##.
        .###....#.
        ..#.#..#.#
        #...##.#..
    
        Tile 1171:
        ####...##.
        #..##.#..#
        ##.#..#.#.
        .###.####.
        ..###.####
        .##....##.
        .#...####.
        #.##.####.
        ####..#...
        .....##...
    
        Tile 1427:
        ###.##.#..
        .#..#.##..
        .#.##.#..#
        #.#.#.##.#
        ....#...##
        ...##..##.
        ...#.#####
        .#.####.#.
        ..#..###.#
        ..##.#..#.
    
        Tile 1489:
        ##.#.#....
        ..##...#..
        .##..##...
        ..#...#...
        #####...#.
        #..#.#.#.#
        ...#.#.#..
        ##.#...##.
        ..##.##.##
        ###.##.#..
        
        Tile 2473:
        #....####.
        #..#.##...
        #.##..#...
        ######.#.#
        .#...#.#.#
        .#########
        .###.#..#.
        ########.#
        ##...##.#.
        ..###.#.#.
    
        Tile 2971:
        ..#.#....#
        #...###...
        #.#.###...
        ##.##..#..
        .#####..##
        .#..####.#
        #..#.#..#.
        ..####.###
        ..#.#.###.
        ...#.#.#.#
    
        Tile 2729:
        ...#.#.#.#
        ####.#....
        ..#.#.....
        ....#..#.#
        .##..##.#.
        .#.####...
        ####.#.#..
        ##.####...
        ##..#.##..
        #.##...##.
    
        Tile 3079:
        #.#.#####.
        .#..######
        ..#.......
        ######....
        ####.#..#.
        .#...#.##.
        #.#####.##
        ..#.###...
        ..#.......
        ..#.###...
    "};

    static EXAMPLE1_COMBINED_IMAGE: &str = indoc! {"
        ...###...##...#...#..###
        .#.###..##..##..####.##.
        #.##..#..#...#..####...#
        #####..#####...###....##
        #..####...#.#.#.###.###.
        ..#.#..#..#.#.#.####.###
        .####.###.#...###.#..#.#
        .#.#.###.##.##.#..#.##..
        ###.#...#..#.##.######..
        .#.#....#.##.#...###.##.
        ...#..#..#.#.##..###.###
        ##..##.#...#...#.#.#.#..
        #.####....##..########.#
        ###.#.#...#.######.#..##
        #.####..#.####.#.#.###..
        #..#.##..#..###.#.##....
        .####...#..#.....#......
        ....#..#...##..#.#.###..
        ...########.#....#####.#
        ##.#....#.##.####...#.##
        ###.#####...#.#####.#..#
        ##.##.###.#.#..######...
        ###....#.#....#..#......
        .#.#..#.##...#.##..#####
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 20899048083289);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 111936085519519);
    }

    #[test]
    fn test_example1_combined_image() {
        let array = CameraArray::parse(EXAMPLE1);
        let image = array.solve_for_combined_image();
        println!("{}", image.to_string());
        assert_eq!(image.to_string(), EXAMPLE1_COMBINED_IMAGE.trim());
    }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}