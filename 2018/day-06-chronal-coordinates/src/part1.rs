use std::collections::HashMap;
use crate::grid::*;

pub fn run(locations: &[Point]) -> usize {
    let grid = classify_grid(locations);
    let points = points_by_location(&grid);
    let areas = area_for_locations(&points, &grid);

    return areas.iter().map(|(_id, area)| {
        match area {
            Area::Infinite => None,
            Area::Finite(size) => Some(*size),
        }
    }).filter(|a| a.is_some()).map(|a| a.unwrap()).max().unwrap();
}

fn classify_grid(locations: &[Point]) -> Grid<Classification> {
    let bounds = Grid::<Classification>::find_bounds(locations);
    let mut grid: Grid<Classification> = Grid::new(bounds, Classification::Tie);
    grid.mutate_points(&|p: &Point| classify_point(p, locations));
    return grid;
}

fn classify_point(point: &Point, locations: &[Point]) -> Classification {
    let mut curr_dist = None;
    let mut curr_classification = Classification::Tie; // placeholder, not real

    for (id, location) in locations.iter().enumerate() {
        if point == location {
            return Classification::Location(id as u16);
        } else {
            let dist = point.manhattan_distance(&location);
            if curr_dist == None || dist < curr_dist.unwrap() {
                curr_dist = Some(dist);
                curr_classification = Classification::Owned(id as u16);
            } else if dist == curr_dist.unwrap() {
                curr_classification = Classification::Tie;
            }
        }
    }

    return curr_classification;
}

fn points_by_location(grid: &Grid<Classification>) -> HashMap<u16, Vec<Point>> {
    let mut res = HashMap::new();

    for point in grid.points() {
        let classification = grid.get(&point);
        match classification {
            Classification::Location(id) | Classification::Owned(id) => {
                let points = res.entry(*id).or_insert(vec![]);
                points.push(point);
            },
            Classification::Tie => {},
        }
    }

    return res;
}

#[derive(Debug, PartialEq)]
enum Area {
    Finite(usize),
    Infinite,
}

fn area_for_locations(points_by_location: &HashMap<u16, Vec<Point>>, grid: &Grid<Classification>) -> HashMap<u16, Area> {
    return points_by_location.iter().map(|(&id, points)| (id, area_for_points(&points, grid))).collect();
}

fn area_for_points(points: &[Point], grid: &Grid<Classification>) -> Area {
    if points.iter().any(|p| grid.on_boundary(&p)) {
        return Area::Infinite;
    } else {
        return Area::Finite(points.len());
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Classification {
    Location(u16),
    Owned(u16),
    Tie,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_classify_grid() {
        let locations = vec![
            Point::new(1, 1),
            Point::new(1, 6),
            Point::new(8, 3),
            Point::new(3, 4),
            Point::new(5, 5),
            Point::new(8, 9),
        ];

        let grid = classify_grid(&locations);

        // Aaaa.ccc
        assert_eq!(*grid.get(&Point::new(1, 1)), Classification::Location(0));
        assert_eq!(*grid.get(&Point::new(2, 1)), Classification::Owned(0));
        assert_eq!(*grid.get(&Point::new(3, 1)), Classification::Owned(0));
        assert_eq!(*grid.get(&Point::new(4, 1)), Classification::Owned(0));
        assert_eq!(*grid.get(&Point::new(5, 1)), Classification::Tie);
        assert_eq!(*grid.get(&Point::new(6, 1)), Classification::Owned(2));
        assert_eq!(*grid.get(&Point::new(7, 1)), Classification::Owned(2));
        assert_eq!(*grid.get(&Point::new(8, 1)), Classification::Owned(2));

        // aaddeccc
        assert_eq!(*grid.get(&Point::new(1, 2)), Classification::Owned(0));
        assert_eq!(*grid.get(&Point::new(2, 2)), Classification::Owned(0));
        assert_eq!(*grid.get(&Point::new(3, 2)), Classification::Owned(3));
        assert_eq!(*grid.get(&Point::new(4, 2)), Classification::Owned(3));
        assert_eq!(*grid.get(&Point::new(5, 2)), Classification::Owned(4));
        assert_eq!(*grid.get(&Point::new(6, 2)), Classification::Owned(2));
        assert_eq!(*grid.get(&Point::new(7, 2)), Classification::Owned(2));
        assert_eq!(*grid.get(&Point::new(8, 2)), Classification::Owned(2));

        // adddeccC
        assert_eq!(*grid.get(&Point::new(1, 3)), Classification::Owned(0));
        assert_eq!(*grid.get(&Point::new(2, 3)), Classification::Owned(3));
        assert_eq!(*grid.get(&Point::new(3, 3)), Classification::Owned(3));
        assert_eq!(*grid.get(&Point::new(4, 3)), Classification::Owned(3));
        assert_eq!(*grid.get(&Point::new(5, 3)), Classification::Owned(4));
        assert_eq!(*grid.get(&Point::new(6, 3)), Classification::Owned(2));
        assert_eq!(*grid.get(&Point::new(7, 3)), Classification::Owned(2));
        assert_eq!(*grid.get(&Point::new(8, 3)), Classification::Location(2));

        // .dDdeecc
        assert_eq!(*grid.get(&Point::new(1, 4)), Classification::Tie);
        assert_eq!(*grid.get(&Point::new(2, 4)), Classification::Owned(3));
        assert_eq!(*grid.get(&Point::new(3, 4)), Classification::Location(3));
        assert_eq!(*grid.get(&Point::new(4, 4)), Classification::Owned(3));
        assert_eq!(*grid.get(&Point::new(5, 4)), Classification::Owned(4));
        assert_eq!(*grid.get(&Point::new(6, 4)), Classification::Owned(4));
        assert_eq!(*grid.get(&Point::new(7, 4)), Classification::Owned(2));
        assert_eq!(*grid.get(&Point::new(8, 4)), Classification::Owned(2));

        // b.deEeec
        assert_eq!(*grid.get(&Point::new(1, 5)), Classification::Owned(1));
        assert_eq!(*grid.get(&Point::new(2, 5)), Classification::Tie);
        assert_eq!(*grid.get(&Point::new(3, 5)), Classification::Owned(3));
        assert_eq!(*grid.get(&Point::new(4, 5)), Classification::Owned(4));
        assert_eq!(*grid.get(&Point::new(5, 5)), Classification::Location(4));
        assert_eq!(*grid.get(&Point::new(6, 5)), Classification::Owned(4));
        assert_eq!(*grid.get(&Point::new(7, 5)), Classification::Owned(4));
        assert_eq!(*grid.get(&Point::new(8, 5)), Classification::Owned(2));

        // Bb.eeee.
        assert_eq!(*grid.get(&Point::new(1, 6)), Classification::Location(1));
        assert_eq!(*grid.get(&Point::new(2, 6)), Classification::Owned(1));
        assert_eq!(*grid.get(&Point::new(3, 6)), Classification::Tie);
        assert_eq!(*grid.get(&Point::new(4, 6)), Classification::Owned(4));
        assert_eq!(*grid.get(&Point::new(5, 6)), Classification::Owned(4));
        assert_eq!(*grid.get(&Point::new(6, 6)), Classification::Owned(4));
        assert_eq!(*grid.get(&Point::new(7, 6)), Classification::Owned(4));
        assert_eq!(*grid.get(&Point::new(8, 6)), Classification::Tie);

        // bb.eeeff
        assert_eq!(*grid.get(&Point::new(1, 7)), Classification::Owned(1));
        assert_eq!(*grid.get(&Point::new(2, 7)), Classification::Owned(1));
        assert_eq!(*grid.get(&Point::new(3, 7)), Classification::Tie);
        assert_eq!(*grid.get(&Point::new(4, 7)), Classification::Owned(4));
        assert_eq!(*grid.get(&Point::new(5, 7)), Classification::Owned(4));
        assert_eq!(*grid.get(&Point::new(6, 7)), Classification::Owned(4));
        assert_eq!(*grid.get(&Point::new(7, 7)), Classification::Owned(5));
        assert_eq!(*grid.get(&Point::new(8, 7)), Classification::Owned(5));

        // bb.eefff
        assert_eq!(*grid.get(&Point::new(1, 8)), Classification::Owned(1));
        assert_eq!(*grid.get(&Point::new(2, 8)), Classification::Owned(1));
        assert_eq!(*grid.get(&Point::new(3, 8)), Classification::Tie);
        assert_eq!(*grid.get(&Point::new(4, 8)), Classification::Owned(4));
        assert_eq!(*grid.get(&Point::new(5, 8)), Classification::Owned(4));
        assert_eq!(*grid.get(&Point::new(6, 8)), Classification::Owned(5));
        assert_eq!(*grid.get(&Point::new(7, 8)), Classification::Owned(5));
        assert_eq!(*grid.get(&Point::new(8, 8)), Classification::Owned(5));

        // bb.ffffF
        assert_eq!(*grid.get(&Point::new(1, 9)), Classification::Owned(1));
        assert_eq!(*grid.get(&Point::new(2, 9)), Classification::Owned(1));
        assert_eq!(*grid.get(&Point::new(3, 9)), Classification::Tie);
        assert_eq!(*grid.get(&Point::new(4, 9)), Classification::Owned(5));
        assert_eq!(*grid.get(&Point::new(5, 9)), Classification::Owned(5));
        assert_eq!(*grid.get(&Point::new(6, 9)), Classification::Owned(5));
        assert_eq!(*grid.get(&Point::new(7, 9)), Classification::Owned(5));
        assert_eq!(*grid.get(&Point::new(8, 9)), Classification::Location(5));
    }

    #[test]
    fn test_calculate_area_by_location() {
        let locations = vec![
            Point::new(1, 1),
            Point::new(1, 6),
            Point::new(8, 3),
            Point::new(3, 4),
            Point::new(5, 5),
            Point::new(8, 9),
        ];

        let grid = classify_grid(&locations);
        let points = points_by_location(&grid);
        let areas = area_for_locations(&points, &grid);

        assert_eq!(areas[&0], Area::Infinite);
        assert_eq!(areas[&1], Area::Infinite);
        assert_eq!(areas[&2], Area::Infinite);
        assert_eq!(areas[&3], Area::Finite(9));
        assert_eq!(areas[&4], Area::Finite(17));
        assert_eq!(areas[&5], Area::Infinite);
    }
}
