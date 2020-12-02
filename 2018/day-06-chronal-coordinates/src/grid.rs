#[derive(Debug, PartialEq)]
pub struct Point {
    x: u16,
    y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Point {
        return Point {
            x: x,
            y: y,
        };
    }

    pub fn manhattan_distance(&self, other: &Point) -> usize {
        let dx = ((self.x as i32) - (other.x as i32)).abs() as usize;
        let dy = ((self.y as i32) - (other.y as i32)).abs() as usize;
        return dx + dy;
    }
}

#[derive(Debug)]
pub struct Grid<T> {
    x_offset: u16,
    y_offset: u16,
    width: u16,
    height: u16,
    size: usize,
    pub values: Vec<T>,
}

impl<T> Grid<T> where T: std::clone::Clone {
    pub fn new(bounds: (u16, u16, u16, u16), default: T) -> Grid<T> {
        let (min_x, max_x, min_y, max_y) = bounds;

        assert!(max_x >= min_x);
        assert!(max_y >= min_y);

        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;
        let size = (width as usize) * (height as usize);

        let values: Vec<T> = vec![default; size];

        return Grid {
            x_offset: min_x,
            y_offset: min_y,
            width: width,
            height: height,
            size: size,
            values: values,
        };
    }

    pub fn find_bounds(points: &[Point]) -> (u16, u16, u16, u16) {
        let min_x = points.iter().map(|p| p.x).min().unwrap();
        let max_x = points.iter().map(|p| p.x).max().unwrap();
        let min_y = points.iter().map(|p| p.y).min().unwrap();
        let max_y = points.iter().map(|p| p.y).max().unwrap();
        return (min_x, max_x, min_y, max_y);
    }

    pub fn points(&self) -> GridIterator<T> {
        return GridIterator {
            i: 0,
            grid: self,
        };
    }

    pub fn mutate_points(&mut self, f: &Fn(&Point) -> T) {
        for i in 0..self.size {
            let point = self.index_to_point(i);
            let val = f(&point);
            self.values[i] = val;
        }
    }

    pub fn get(&self, point: &Point) -> &T {
        let i = self.point_to_index(point);
        return &self.values[i];
    }

    pub fn set(&mut self, point: &Point, value: T) {
        let i = self.point_to_index(point);
        self.values[i] = value;
    }

    pub fn on_boundary(&self, point: &Point) -> bool {
        return point.x == self.x_offset ||
            point.x == self.x_offset + self.width - 1 ||
            point.y == self.y_offset ||
            point.y == self.y_offset + self.height - 1;
    }
}

impl <T> Grid<T> {
    fn index_to_point(&self, i: usize) -> Point {
        let dx = (i % (self.width as usize)) as u16;
        let dy = (i / (self.width as usize)) as u16;

        let x = dx + self.x_offset;
        let y = dy + self.y_offset;

        return Point {x: x, y: y};
    }

    fn point_to_index(&self, point: &Point) -> usize {
        let dx = point.x - self.x_offset;
        let dy = point.y - self.y_offset;
        return dx as usize + ((dy as usize) * (self.width as usize));
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    i: usize,
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.i >= self.grid.size {
            return None;
        } else {
            let output = self.grid.index_to_point(self.i);
            self.i += 1;
            return Some(output);
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_find_bounds() {
        let points = vec![
            Point::new(1, 4),
            Point::new(2, 3),
            Point::new(1, 7),
            Point::new(3, 2),
        ];
        let bounds = Grid::<u16>::find_bounds(&points);
        assert_eq!(bounds, (1, 3, 2, 7));
    }

    #[test]
    fn test_new_grid() {
        let bounds = (1, 3, 2, 5);
        let grid = Grid::<u16>::new(bounds, 0);

        assert_eq!(grid.x_offset, 1);
        assert_eq!(grid.y_offset, 2);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 4);
        assert_eq!(grid.size, 12);
    }

    #[test]
    fn test_point_to_index() {
        let bounds = (1, 3, 2, 5);
        let grid = Grid::<u16>::new(bounds, 0);

        assert_eq!(grid.point_to_index(&Point::new(1, 2)), 0);
        assert_eq!(grid.point_to_index(&Point::new(2, 2)), 1);
        assert_eq!(grid.point_to_index(&Point::new(3, 2)), 2);
        assert_eq!(grid.point_to_index(&Point::new(1, 3)), 3);
        assert_eq!(grid.point_to_index(&Point::new(2, 3)), 4);
        assert_eq!(grid.point_to_index(&Point::new(3, 3)), 5);
        assert_eq!(grid.point_to_index(&Point::new(1, 4)), 6);
        assert_eq!(grid.point_to_index(&Point::new(2, 4)), 7);
        assert_eq!(grid.point_to_index(&Point::new(3, 4)), 8);
        assert_eq!(grid.point_to_index(&Point::new(1, 5)), 9);
        assert_eq!(grid.point_to_index(&Point::new(2, 5)), 10);
        assert_eq!(grid.point_to_index(&Point::new(3, 5)), 11);
    }

    #[test]
    fn test_index_to_point() {
        let bounds = (1, 3, 2, 5);
        let grid = Grid::<u16>::new(bounds, 0);

        assert_eq!(grid.index_to_point(0), Point::new(1, 2));
        assert_eq!(grid.index_to_point(1), Point::new(2, 2));
        assert_eq!(grid.index_to_point(2), Point::new(3, 2));
        assert_eq!(grid.index_to_point(3), Point::new(1, 3));
        assert_eq!(grid.index_to_point(4), Point::new(2, 3));
        assert_eq!(grid.index_to_point(5), Point::new(3, 3));
        assert_eq!(grid.index_to_point(6), Point::new(1, 4));
        assert_eq!(grid.index_to_point(7), Point::new(2, 4));
        assert_eq!(grid.index_to_point(8), Point::new(3, 4));
        assert_eq!(grid.index_to_point(9), Point::new(1, 5));
        assert_eq!(grid.index_to_point(10), Point::new(2, 5));
        assert_eq!(grid.index_to_point(11), Point::new(3, 5));
    }

    #[test]
    fn test_points() {
        let bounds = (1, 3, 2, 5);
        let grid = Grid::<u16>::new(bounds, 0);

        let points: Vec<Point> = grid.points().collect();
        assert_eq!(
            points,
            vec![
                Point::new(1, 2),
                Point::new(2, 2),
                Point::new(3, 2),
                Point::new(1, 3),
                Point::new(2, 3),
                Point::new(3, 3),
                Point::new(1, 4),
                Point::new(2, 4),
                Point::new(3, 4),
                Point::new(1, 5),
                Point::new(2, 5),
                Point::new(3, 5),
            ],
        );
    }

    #[test]
    fn test_get() {
        let bounds = (1, 3, 2, 5);
        let grid = Grid::<u16>::new(bounds, 0);
        assert_eq!(*grid.get(&Point::new(1, 2)), 0);
    }

    #[test]
    fn test_set() {
        let bounds = (1, 3, 2, 5);
        let mut grid = Grid::<u16>::new(bounds, 0);
        grid.set(&Point::new(1, 2), 1);
        assert_eq!(*grid.get(&Point::new(1, 2)), 1);
        assert_eq!(*grid.get(&Point::new(2, 2)), 0);
    }

    #[test]
    fn test_on_boundary() {
        let bounds = (1, 3, 2, 5);
        let grid = Grid::<u16>::new(bounds, 0);

        assert_eq!(grid.on_boundary(&Point::new(1, 2)), true);
        assert_eq!(grid.on_boundary(&Point::new(2, 2)), true);
        assert_eq!(grid.on_boundary(&Point::new(3, 2)), true);
        assert_eq!(grid.on_boundary(&Point::new(1, 3)), true);
        assert_eq!(grid.on_boundary(&Point::new(2, 3)), false);
        assert_eq!(grid.on_boundary(&Point::new(3, 3)), true);
        assert_eq!(grid.on_boundary(&Point::new(1, 4)), true);
        assert_eq!(grid.on_boundary(&Point::new(2, 4)), false);
        assert_eq!(grid.on_boundary(&Point::new(3, 4)), true);
        assert_eq!(grid.on_boundary(&Point::new(1, 5)), true);
        assert_eq!(grid.on_boundary(&Point::new(2, 5)), true);
        assert_eq!(grid.on_boundary(&Point::new(3, 5)), true);
    }

    #[test]
    fn test_mutate_points() {
        let bounds = (0, 2, 0, 3);
        let mut grid = Grid::<u16>::new(bounds, 0);
        grid.mutate_points(&|p: &Point| p.x + p.y);
        assert_eq!(grid.values, vec![0, 1, 2, 1, 2, 3, 2, 3, 4, 3, 4, 5]);
    }

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(Point::new(0, 0).manhattan_distance(&Point::new(0, 1)), 1);
        assert_eq!(Point::new(0, 0).manhattan_distance(&Point::new(1, 1)), 2);
        assert_eq!(Point::new(0, 0).manhattan_distance(&Point::new(0, 5)), 5);
        assert_eq!(Point::new(0, 0).manhattan_distance(&Point::new(5, 0)), 5);
        assert_eq!(Point::new(0, 0).manhattan_distance(&Point::new(5, 5)), 10);
        assert_eq!(Point::new(5, 5).manhattan_distance(&Point::new(0, 0)), 10);
        assert_eq!(Point::new(1, 2).manhattan_distance(&Point::new(3, 4)), 4);
    }
}
