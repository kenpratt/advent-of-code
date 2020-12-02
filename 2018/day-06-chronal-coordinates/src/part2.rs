use crate::grid::*;

pub fn run(locations: &[Point]) -> usize {
    let grid = build_grid(locations);
    return area_below(&grid, 10000);
}

fn build_grid(locations: &[Point]) -> Grid<usize> {
    let bounds = Grid::<usize>::find_bounds(locations);
    let mut grid: Grid<usize> = Grid::new(bounds, 0);
    grid.mutate_points(&|p: &Point| total_distance(p, locations));
    return grid;
}

fn total_distance(point: &Point, locations: &[Point]) -> usize {
    return locations.iter().map(|l| l.manhattan_distance(point)).sum();
}

fn area_below(grid: &Grid<usize>, threshold: usize) -> usize {
    return grid.values.iter().filter(|&v| *v < threshold).count();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_example_grid() {
        let locations = vec![
            Point::new(1, 1),
            Point::new(1, 6),
            Point::new(8, 3),
            Point::new(3, 4),
            Point::new(5, 5),
            Point::new(8, 9),
        ];

        let grid = build_grid(&locations);
        assert_eq!(*grid.get(&Point::new(4, 3)), 30);

        assert_eq!(area_below(&grid, 32), 16);
    }
}
