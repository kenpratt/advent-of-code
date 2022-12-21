use std::cmp;
use std::ops::RangeInclusive;
use std::vec;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }

    pub fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

pub fn point(x: i32, y: i32) -> Point {
    Point { x, y }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Segment {
    pub p1: Point,
    pub p2: Point,
}

impl Segment {
    fn x_range(&self) -> RangeInclusive<i32> {
        if self.p1.x <= self.p2.x {
            self.p1.x..=self.p2.x
        } else {
            self.p2.x..=self.p1.x
        }
    }

    fn y_range(&self) -> RangeInclusive<i32> {
        if self.p1.y <= self.p2.y {
            self.p1.y..=self.p2.y
        } else {
            self.p2.y..=self.p1.y
        }
    }

    fn contains(&self, other: &Point) -> bool {
        // is the point inside the bounding box of our points?
        if self.x_range().contains(&other.x) && self.y_range().contains(&other.y) {
            // it's in the box, check if it's on the line between the points
            self.line().contains(other)
        } else {
            // it's not in the box
            false
        }
    }

    fn line(&self) -> Line {
        line(&self.p1, &self.p2)
    }

    pub fn edge(&self, facing: Facing) -> Edge {
        edge(self.line(), facing)
    }

    fn bisect(&self, edge: &Edge) -> SegmentBisection {
        use InEdge::*;
        use SegmentBisection::*;

        let line = self.line();
        let edge_bisect_res = line.bisect(&edge);

        match edge_bisect_res {
            Some(((b1p, b1e), (b2p, b2e))) => {
                // Test if the bisection point is inside the segment or not
                match ((self.contains(&b1p), &b1e), (self.contains(&b2p), &b2e)) {
                    ((true, _), (true, _)) => {
                        // normal bisection case
                        if self.p1 <= self.p2 {
                            let first = segment(self.p1, b1p);
                            let second = segment(b2p, self.p2);
                            Bisection((first, b1e), (second, b2e))
                        } else {
                            // segment points are in reverse order, but bisection results
                            // will be in normal order since lines are normalized.
                            let first = segment(self.p1, b2p);
                            let second = segment(b1p, self.p2);
                            Bisection((first, b2e), (second, b1e))
                        }
                    }
                    ((true, Inside), (false, Outside)) | ((false, Outside), (true, Inside)) => {
                        // segment borders the edge, but doesn't reach inside
                        Contained(Inside)
                    }
                    ((true, Outside), (false, Inside)) | ((false, Inside), (true, Outside)) => {
                        // edge borders the segment, but does not intersect
                        Contained(Outside)
                    }
                    ((false, _), (false, _)) => {
                        // bisection point is further away from segment, segment could be inside or outside
                        if edge.contains(&self.p1) {
                            Contained(Inside)
                        } else {
                            Contained(Outside)
                        }
                    }
                    _ => panic!("Unreachable"),
                }
            }
            None => {
                // Must be parallel lines
                if edge.contains(&self.p1) {
                    Contained(Inside)
                } else {
                    Contained(Outside)
                }
            }
        }
    }
}

pub fn segment(p1: Point, p2: Point) -> Segment {
    Segment { p1, p2 }
}

#[derive(Debug, Eq, PartialEq)]
enum SegmentBisection {
    Bisection((Segment, InEdge), (Segment, InEdge)),
    Contained(InEdge),
}

impl SegmentBisection {
    fn is_bisection(&self) -> bool {
        match self {
            SegmentBisection::Bisection(_, _) => true,
            _ => false,
        }
    }

    fn first(&self) -> Option<&(Segment, InEdge)> {
        match self {
            SegmentBisection::Bisection(r, _) => Some(r),
            _ => None,
        }
    }

    fn second(&self) -> Option<&(Segment, InEdge)> {
        match self {
            SegmentBisection::Bisection(_, r) => Some(r),
            _ => None,
        }
    }

    fn contained(&self) -> Option<&InEdge> {
        match self {
            SegmentBisection::Contained(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum BeforeOrAfter {
    Before,
    After,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Line {
    Vertical(i32),
    Horizontal(i32),
    Diagonal(i32, i32),
}

impl Line {
    fn new(p1: &Point, p2: &Point) -> Self {
        use Line::*;

        if p1.x == p2.x {
            Vertical(p1.x)
        } else if p1.y == p2.y {
            Horizontal(p1.y)
        } else {
            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            if dx.abs() != dy.abs() {
                panic!("Currently only support 45 degree lines: {}, {}", dx, dy);
            }
            let slope = dy / dx;
            assert_eq!(slope.abs(), 1);
            let y_intercept = p1.y - slope * p1.x;
            Diagonal(slope, y_intercept)
        }
    }

    fn next_point(&self, p: &Point, d: &BeforeOrAfter) -> Point {
        use BeforeOrAfter::*;

        match d {
            Before => self.point_before(p),
            After => self.point_after(p),
        }
    }

    fn point_before(&self, p: &Point) -> Point {
        use Line::*;

        match self {
            Vertical(_) => point(p.x, p.y - 1),
            Horizontal(_) => point(p.x - 1, p.y),
            Diagonal(s, _) => point(p.x - 1, p.y - s),
        }
    }

    fn point_after(&self, p: &Point) -> Point {
        use Line::*;

        match self {
            Vertical(_) => point(p.x, p.y + 1),
            Horizontal(_) => point(p.x + 1, p.y),
            Diagonal(s, _) => point(p.x + 1, p.y + s),
        }
    }

    fn intersect(&self, other: &Line) -> Intersection {
        use Intersection::*;
        use Line::*;

        match (self, other) {
            (Vertical(_), Vertical(_)) => None,
            (Horizontal(_), Horizontal(_)) => None,
            (Diagonal(s1, _), Diagonal(s2, _)) if s1 == s2 => None, // same slope
            (Vertical(x), Horizontal(y)) | (Horizontal(y), Vertical(x)) => At(point(*x, *y)),
            (Vertical(x), Diagonal(s, yi)) | (Diagonal(s, yi), Vertical(x)) => {
                let y = yi + s * x;
                At(point(*x, y))
            }
            (Horizontal(y), Diagonal(s, yi)) | (Diagonal(s, yi), Horizontal(y)) => {
                let x = (y - yi) * s;
                At(point(x, *y))
            }
            (Diagonal(s1, yi1), Diagonal(s2, yi2)) => {
                let yid = yi2 - yi1;
                let sd = s1 - s2;

                if yid % sd == 0 {
                    // intersection is at a point
                    let x = yid / sd;
                    let y = yi1 + s1 * x;
                    At(point(x, y))
                } else {
                    // intersection is between two points, so return two closest
                    // points on the self line
                    let fx = yid as f64 / sd as f64;

                    let first_x = fx.floor() as i32;
                    let first_y = *yi1 + *s1 * first_x;

                    let second_x = fx.ceil() as i32;
                    let second_y = *yi1 + *s1 * second_x;

                    Between(point(first_x, first_y), point(second_x, second_y))
                }
            }
        }
    }

    fn bisect(&self, edge: &Edge) -> Option<((Point, InEdge), (Point, InEdge))> {
        edge.bisect_line(&self)
    }

    fn contains(&self, point: &Point) -> bool {
        use Line::*;
        match self {
            Vertical(x) => x == &point.x,
            Horizontal(y) => y == &point.y,
            Diagonal(s, yi) => (yi + s * point.x) == point.y,
        }
    }
}

fn line(p1: &Point, p2: &Point) -> Line {
    Line::new(p1, p2)
}

#[derive(Debug, Eq, PartialEq)]
enum Intersection {
    None,
    At(Point),
    Between(Point, Point),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Facing {
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Edge {
    line: Line,
    facing: Facing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InEdge {
    Inside,
    Outside,
}

impl Edge {
    fn away_from_edge(&self, line: &Line) -> BeforeOrAfter {
        use BeforeOrAfter::*;

        let (slope, _) = self.slope_and_intercept();
        match (slope, &self.facing) {
            (1, Facing::Right) => match line {
                Line::Vertical(_) => After,
                _ => Before,
            },
            (1, Facing::Left) => match line {
                Line::Vertical(_) => Before,
                _ => After,
            },
            (-1, Facing::Right) => Before,
            (-1, Facing::Left) => After,
            _ => panic!("Unreachable"),
        }
    }

    fn bisect_line(&self, line: &Line) -> Option<((Point, InEdge), (Point, InEdge))> {
        use BeforeOrAfter::*;

        match line.intersect(&self.line) {
            Intersection::None => None,
            Intersection::At(p) => {
                let d = self.away_from_edge(line);
                let o = line.next_point(&p, &d);
                if o < p {
                    Some(((o, InEdge::Outside), (p, InEdge::Inside)))
                } else {
                    Some(((p, InEdge::Inside), (o, InEdge::Outside)))
                }
            }
            Intersection::Between(p1, p2) => match self.away_from_edge(line) {
                Before => Some(((p1, InEdge::Outside), (p2, InEdge::Inside))),
                After => Some(((p1, InEdge::Inside), (p2, InEdge::Outside))),
            },
        }
    }

    fn contains(&self, point: &Point) -> bool {
        let (s, yi) = self.slope_and_intercept();
        let y = yi + s * point.x;

        match (s, &self.facing) {
            (1, Facing::Right) | (-1, Facing::Left) => point.y <= y, // below the edge
            (-1, Facing::Right) | (1, Facing::Left) => point.y >= y, // above the edge
            _ => panic!("Unreachable"),
        }
    }

    fn slope_and_intercept(&self) -> (&i32, &i32) {
        if let Line::Diagonal(s, yi) = &self.line {
            (s, yi)
        } else {
            panic!("Unreachable")
        }
    }
}

fn edge(line: Line, facing: Facing) -> Edge {
    match &line {
        Line::Diagonal(_, _) => Edge { line, facing },
        _ => panic!(
            "For now only diagonal lines are supported for edges: {:?} {:?}",
            line, facing
        ),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Polygon {
    pub points: Vec<Point>,
}

impl Polygon {
    fn from_segments(segments: &[Segment]) -> Self {
        if segments.len() < 3 {
            panic!("Need at least 3 segments to make a polygone");
        }

        // ensure segments are contiguous
        let last_index = segments.len() - 1;
        segments.iter().enumerate().all(|(i, s)| {
            let j = if i == last_index { 0 } else { i + 1 };
            s.p2 == segments[j].p1
        });

        // the unique() is to filter out extra single-point segments
        let points = segments.iter().map(|s| s.p1).unique().collect();
        Self { points }
    }

    // returns (outside, inside)
    pub fn bisect(&self, edge: &Edge) -> (Option<Polygon>, Option<Polygon>) {
        use InEdge::*;

        let segments = self.segments();

        let bisection_results: Vec<(Segment, SegmentBisection)> = segments
            .into_iter()
            .map(|s| {
                let r = s.bisect(edge);
                (s, r)
            })
            .collect();

        let bisections: Vec<(usize, &(Segment, SegmentBisection))> = bisection_results
            .iter()
            .enumerate()
            .filter(|(_i, (_s, r))| r.is_bisection())
            .collect();

        if bisections.is_empty() {
            // polygon is either fully outside, or fully inside the edge
            let contained: Vec<&InEdge> = bisection_results
                .iter()
                .map(|(_s, r)| r.contained().unwrap())
                .collect();
            assert!(contained.iter().all(|e| e == &contained[0]));

            return match &contained[0] {
                Outside => (Some(self.clone()), None),
                Inside => (None, Some(self.clone())),
            };
        }

        // since we early-returned if len == 0, then now it should be 2
        assert_eq!(bisections.len(), 2);

        let res1 = Self::bisected_polygon(&bisections[0], &bisections[1], &bisection_results);
        let res2 = Self::bisected_polygon(&bisections[1], &bisections[0], &bisection_results);

        // return (outside, inside)
        match (res1, res2) {
            ((pi, Inside), (po, Outside)) => (Some(po), Some(pi)),
            ((po, Outside), (pi, Inside)) => (Some(po), Some(pi)),
            _ => panic!("Unreachable"),
        }
    }

    fn bisected_polygon(
        b1: &(usize, &(Segment, SegmentBisection)),
        b2: &(usize, &(Segment, SegmentBisection)),
        results: &[(Segment, SegmentBisection)],
    ) -> (Polygon, InEdge) {
        let (i1, (_s1, r1)) = b1;
        let (i2, (_s2, r2)) = b2;

        let chunks = if i1 < i2 {
            vec![(i1 + 1)..*i2]
        } else {
            vec![(i1 + 1)..results.len(), 0..*i2]
        };

        let mut pieces = vec![*r1.second().unwrap()];
        for ch in chunks {
            pieces.append(
                &mut results[ch]
                    .iter()
                    .map(|(s, r)| (*s, *r.contained().unwrap()))
                    .collect(),
            );
        }
        pieces.push(*r2.first().unwrap());

        // ensure all pieces are either inside or outside
        let contained = pieces[0].1;
        assert!(pieces.iter().all(|(_, e)| e == &contained));

        // map to segments and add a segment for the bisected part
        let mut segments: Vec<Segment> = pieces.into_iter().map(|(s, _)| s).collect();

        let mut conn_path =
            Self::connecting_path(&segments.last().unwrap().p2, &segments.first().unwrap().p1);
        segments.append(&mut conn_path);

        // build polygon
        let polygon = Self::from_segments(&segments);

        (polygon, contained)
    }

    fn segments(&self) -> Vec<Segment> {
        let last_index = self.points.len() - 1;
        self.points
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let j = if i == last_index { 0 } else { i + 1 };
                let q = &self.points[j];
                segment(*p, *q)
            })
            .collect()
    }

    // find a path from p1 to p2 using only horizontal/vertical/diagonal lines,
    fn connecting_path(p1: &Point, p2: &Point) -> Vec<Segment> {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        if dx == 0 || dy == 0 || dx.abs() == dy.abs() {
            // horizontal, vertical, or 45 degree line. clean connection.
            vec![segment(*p1, *p2)]
        } else {
            let shared = cmp::min(dx.abs(), dy.abs());
            let extra = cmp::max(dx.abs(), dy.abs()) - shared;

            // assuming points are in clockwise order, prefer "outside"
            let pm = match (dx.signum(), dy.signum()) {
                (1, 1) => {
                    if dy > dx {
                        // move up first
                        point(p1.x, p1.y + extra)
                    } else {
                        // move diagonally (up-right) first
                        point(p1.x + shared, p1.y + shared)
                    }
                }
                (1, -1) => {
                    if dx > -dy {
                        // move right first
                        point(p1.x + extra, p1.y)
                    } else {
                        // move diagonally (down-right) first
                        point(p1.x + shared, p1.y - shared)
                    }
                }
                (-1, 1) => {
                    if -dx > dy {
                        // move left first
                        point(p1.x - extra, p1.y)
                    } else {
                        // move diagonally (up-left) first
                        point(p1.x - shared, p1.y + shared)
                    }
                }
                (-1, -1) => {
                    if -dy > -dx {
                        // move down first
                        point(p1.x, p1.y - extra)
                    } else {
                        // move diagonally (down-left) first
                        point(p1.x - shared, p1.y - shared)
                    }
                }
                _ => panic!("Unreachable"),
            };

            let mut segments = Self::connecting_path(&pm, p2);
            segments.insert(0, segment(*p1, pm));
            segments
        }
    }
}

pub fn polygon(points: Vec<Point>) -> Polygon {
    Polygon { points }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let p1 = point(2, 2);
        let p2 = point(4, 4);
        assert_eq!(p1.distance(&p2), 2.8284271247461903);
        assert_eq!(p2.distance(&p1), 2.8284271247461903);
    }

    #[test]
    fn test_manhattan_distance() {
        let p1 = point(2, 5);
        let p2 = point(-1, -3);
        assert_eq!(p1.manhattan_distance(&p2), 11);
        assert_eq!(p2.manhattan_distance(&p1), 11);
    }

    #[test]
    fn test_segment_contains() {
        let s1 = segment(point(0, 0), point(5, 5));
        assert_eq!(s1.contains(&point(0, 0)), true);
        assert_eq!(s1.contains(&point(5, 5)), true);
        assert_eq!(s1.contains(&point(2, 2)), true);
        assert_eq!(s1.contains(&point(2, 3)), false);
        assert_eq!(s1.contains(&point(3, 2)), false);
        assert_eq!(s1.contains(&point(6, 6)), false);
        assert_eq!(s1.contains(&point(-1, -1)), false);

        let s2 = segment(point(5, 5), point(0, 0));
        assert_eq!(s2.contains(&point(2, 2)), true);
        assert_eq!(s2.contains(&point(3, 2)), false);
        assert_eq!(s2.contains(&point(6, 6)), false);

        let s3 = segment(point(5, 1), point(5, 3));
        assert_eq!(s3.contains(&point(5, 1)), true);
        assert_eq!(s3.contains(&point(5, 2)), true);
        assert_eq!(s3.contains(&point(5, 3)), true);
        assert_eq!(s3.contains(&point(5, 0)), false);
        assert_eq!(s3.contains(&point(5, 5)), false);
        assert_eq!(s3.contains(&point(4, 2)), false);
        assert_eq!(s3.contains(&point(6, 2)), false);

        let s4 = segment(point(1, 3), point(4, 3));
        assert_eq!(s4.contains(&point(1, 3)), true);
        assert_eq!(s4.contains(&point(2, 3)), true);
        assert_eq!(s4.contains(&point(4, 3)), true);
        assert_eq!(s4.contains(&point(0, 3)), false);
        assert_eq!(s4.contains(&point(5, 3)), false);
        assert_eq!(s4.contains(&point(1, 2)), false);
    }

    #[test]
    fn test_line_constructor() {
        assert_eq!(line(&point(1, 2), &point(1, 5)), Line::Vertical(1));
        assert_eq!(line(&point(1, 5), &point(1, 2)), Line::Vertical(1));
        assert_eq!(line(&point(2, 1), &point(5, 1)), Line::Horizontal(1));
        assert_eq!(line(&point(5, 1), &point(2, 1)), Line::Horizontal(1));
        assert_eq!(line(&point(0, 0), &point(3, 3)), Line::Diagonal(1, 0));
        assert_eq!(line(&point(2, 2), &point(4, 4)), Line::Diagonal(1, 0));
        assert_eq!(line(&point(0, 1), &point(3, 4)), Line::Diagonal(1, 1));
        assert_eq!(line(&point(3, 4), &point(0, 1)), Line::Diagonal(1, 1));
        assert_eq!(line(&point(0, 2), &point(2, 0)), Line::Diagonal(-1, 2));
        assert_eq!(line(&point(2, 0), &point(0, 2)), Line::Diagonal(-1, 2));
    }

    #[test]
    #[should_panic]
    fn test_line_constructor_bad_line() {
        line(&point(0, 0), &point(2, 5));
    }

    #[test]
    fn test_line_intersect() {
        use Intersection::*;

        let v1 = Line::Vertical(0);
        let v2 = Line::Vertical(2);
        let h1 = Line::Horizontal(0);
        let h2 = Line::Horizontal(3);
        let du1 = Line::Diagonal(1, 0);
        let du2 = Line::Diagonal(1, 5);
        let dd1 = Line::Diagonal(-1, 0);
        let dd2 = Line::Diagonal(-1, 5);

        // parallel lines
        assert_eq!(v1.intersect(&v2), None);
        assert_eq!(h1.intersect(&h2), None);
        assert_eq!(du1.intersect(&du2), None);
        assert_eq!(dd1.intersect(&dd2), None);

        // intersecting vertical line
        assert_eq!(v1.intersect(&h1), At(point(0, 0)));
        assert_eq!(v2.intersect(&h2), At(point(2, 3)));
        assert_eq!(h2.intersect(&v2), At(point(2, 3)));
        assert_eq!(v1.intersect(&du1), At(point(0, 0)));
        assert_eq!(v2.intersect(&du2), At(point(2, 7)));
        assert_eq!(du2.intersect(&v2), At(point(2, 7)));
        assert_eq!(v1.intersect(&dd1), At(point(0, 0)));
        assert_eq!(v2.intersect(&dd2), At(point(2, 3)));
        assert_eq!(dd2.intersect(&v2), At(point(2, 3)));

        // intersecting horizontal line
        assert_eq!(h1.intersect(&du1), At(point(0, 0)));
        assert_eq!(h2.intersect(&du2), At(point(-2, 3)));
        assert_eq!(du2.intersect(&h2), At(point(-2, 3)));
        assert_eq!(h1.intersect(&dd1), At(point(0, 0)));
        assert_eq!(h2.intersect(&dd2), At(point(2, 3)));
        assert_eq!(dd2.intersect(&h2), At(point(2, 3)));

        // intersecting diagonal lines
        assert_eq!(du1.intersect(&dd1), At(point(0, 0)));
        assert_eq!(du1.intersect(&dd2), Between(point(2, 2), point(3, 3)));
        assert_eq!(du2.intersect(&dd1), Between(point(-3, 2), point(-2, 3)));
        assert_eq!(du2.intersect(&dd2), At(point(0, 5)));
        assert_eq!(dd1.intersect(&du1), At(point(0, 0)));
        assert_eq!(dd2.intersect(&du1), Between(point(2, 3), point(3, 2)));
        assert_eq!(dd1.intersect(&du2), Between(point(-3, 3), point(-2, 2)));
        assert_eq!(dd2.intersect(&du2), At(point(0, 5)));
    }

    #[test]
    fn test_line_bisect() {
        use InEdge::*;

        assert_eq!(
            None,
            Line::Diagonal(1, 1).bisect(&edge(Line::Diagonal(1, 0), Facing::Right))
        );

        // bisect vertical line
        assert_eq!(
            Some(((point(2, 2), Inside), (point(2, 3), Outside))),
            Line::Vertical(2).bisect(&edge(Line::Diagonal(1, 0), Facing::Right))
        );
        assert_eq!(
            Some(((point(2, 1), Outside), (point(2, 2), Inside))),
            Line::Vertical(2).bisect(&edge(Line::Diagonal(1, 0), Facing::Left))
        );
        assert_eq!(
            Some(((point(2, 1), Outside), (point(2, 2), Inside))),
            Line::Vertical(2).bisect(&edge(Line::Diagonal(-1, 4), Facing::Right))
        );
        assert_eq!(
            Some(((point(2, 2), Inside), (point(2, 3), Outside))),
            Line::Vertical(2).bisect(&edge(Line::Diagonal(-1, 4), Facing::Left))
        );

        // bisect horizontal line
        assert_eq!(
            Some(((point(1, 2), Outside), (point(2, 2), Inside))),
            Line::Horizontal(2).bisect(&edge(Line::Diagonal(1, 0), Facing::Right))
        );
        assert_eq!(
            Some(((point(2, 2), Inside), (point(3, 2), Outside))),
            Line::Horizontal(2).bisect(&edge(Line::Diagonal(1, 0), Facing::Left))
        );
        assert_eq!(
            Some(((point(1, 2), Outside), (point(2, 2), Inside))),
            Line::Horizontal(2).bisect(&edge(Line::Diagonal(-1, 4), Facing::Right))
        );
        assert_eq!(
            Some(((point(2, 2), Inside), (point(3, 2), Outside))),
            Line::Horizontal(2).bisect(&edge(Line::Diagonal(-1, 4), Facing::Left))
        );

        // bisect diagonal line that intersects at a point
        assert_eq!(
            Some(((point(0, 0), Outside), (point(1, 1), Inside))),
            Line::Diagonal(1, 0).bisect(&edge(Line::Diagonal(-1, 2), Facing::Right))
        );
        assert_eq!(
            Some(((point(1, 1), Inside), (point(2, 2), Outside))),
            Line::Diagonal(1, 0).bisect(&edge(Line::Diagonal(-1, 2), Facing::Left))
        );
        assert_eq!(
            Some(((point(0, 2), Outside), (point(1, 1), Inside))),
            Line::Diagonal(-1, 2).bisect(&edge(Line::Diagonal(1, 0), Facing::Right))
        );
        assert_eq!(
            Some(((point(1, 1), Inside), (point(2, 0), Outside))),
            Line::Diagonal(-1, 2).bisect(&edge(Line::Diagonal(1, 0), Facing::Left))
        );

        // bisect diagonal line that intersects between two points
        assert_eq!(
            Some(((point(1, 1), Outside), (point(2, 2), Inside))),
            Line::Diagonal(1, 0).bisect(&edge(Line::Diagonal(-1, 3), Facing::Right))
        );
        assert_eq!(
            Some(((point(1, 1), Inside), (point(2, 2), Outside))),
            Line::Diagonal(1, 0).bisect(&edge(Line::Diagonal(-1, 3), Facing::Left))
        );
        assert_eq!(
            Some(((point(1, 2), Outside), (point(2, 1), Inside))),
            Line::Diagonal(-1, 3).bisect(&edge(Line::Diagonal(1, 0), Facing::Right))
        );
        assert_eq!(
            Some(((point(1, 2), Inside), (point(2, 1), Outside))),
            Line::Diagonal(-1, 3).bisect(&edge(Line::Diagonal(1, 0), Facing::Left))
        );
    }

    #[test]
    fn test_edge_contains() {
        let e1 = edge(Line::Diagonal(1, 0), Facing::Right);
        assert_eq!(true, e1.contains(&point(5, 1)));
        assert_eq!(true, e1.contains(&point(1, -5)));
        assert_eq!(true, e1.contains(&point(0, 0)));
        assert_eq!(false, e1.contains(&point(0, 1)));

        let e2 = edge(Line::Diagonal(1, 0), Facing::Left);
        assert_eq!(true, e2.contains(&point(0, 1)));
        assert_eq!(true, e2.contains(&point(0, 0)));
        assert_eq!(false, e2.contains(&point(1, 0)));

        let e3 = edge(Line::Diagonal(-1, 4), Facing::Right);
        assert_eq!(true, e3.contains(&point(5, 1)));
        assert_eq!(true, e3.contains(&point(1, 5)));
        assert_eq!(true, e3.contains(&point(2, 2)));
        assert_eq!(false, e3.contains(&point(0, 0)));

        let e4 = edge(Line::Diagonal(-1, 4), Facing::Left);
        assert_eq!(true, e4.contains(&point(0, 1)));
        assert_eq!(true, e4.contains(&point(2, 2)));
        assert_eq!(false, e4.contains(&point(3, 3)));
    }

    #[test]
    fn test_segment_bisect() {
        use InEdge::*;
        use SegmentBisection::*;

        let s = segment(point(0, 0), point(5, 5));

        // bisection case
        assert_eq!(
            Bisection(
                (segment(point(0, 0), point(1, 1)), InEdge::Outside),
                (segment(point(2, 2), point(5, 5)), InEdge::Inside)
            ),
            s.bisect(&edge(Line::Diagonal(-1, 4), Facing::Right))
        );
        assert_eq!(
            Bisection(
                (segment(point(0, 0), point(2, 2)), InEdge::Inside),
                (segment(point(3, 3), point(5, 5)), InEdge::Outside)
            ),
            s.bisect(&edge(Line::Diagonal(-1, 4), Facing::Left))
        );
        assert_eq!(
            Bisection(
                (segment(point(0, 0), point(2, 2)), InEdge::Outside),
                (segment(point(3, 3), point(5, 5)), InEdge::Inside)
            ),
            s.bisect(&edge(Line::Diagonal(-1, 5), Facing::Right))
        );
        assert_eq!(
            Bisection(
                (segment(point(0, 0), point(2, 2)), InEdge::Inside),
                (segment(point(3, 3), point(5, 5)), InEdge::Outside)
            ),
            s.bisect(&edge(Line::Diagonal(-1, 5), Facing::Left))
        );
        assert_eq!(
            Bisection(
                (segment(point(0, 0), point(0, 0)), InEdge::Inside),
                (segment(point(1, 1), point(5, 5)), InEdge::Outside)
            ),
            s.bisect(&edge(Line::Diagonal(-1, 0), Facing::Left))
        );
        assert_eq!(
            Bisection(
                (segment(point(0, 14), point(0, 14)), InEdge::Inside),
                (segment(point(1, 15), point(6, 20)), InEdge::Outside)
            ),
            segment(point(0, 14), point(6, 20)).bisect(&edge(Line::Diagonal(-1, 14), Facing::Left))
        );

        // fully outside
        assert_eq!(
            Contained(Outside),
            s.bisect(&edge(Line::Diagonal(-1, 11), Facing::Right))
        );
        assert_eq!(
            Contained(Outside),
            s.bisect(&edge(Line::Diagonal(-1, -1), Facing::Left))
        );
        assert_eq!(
            Contained(Outside),
            s.bisect(&edge(Line::Diagonal(1, -1), Facing::Right))
        );
        assert_eq!(
            Contained(Outside),
            s.bisect(&edge(Line::Diagonal(1, 1), Facing::Left))
        );
        assert_eq!(
            Contained(Outside),
            s.bisect(&edge(Line::Diagonal(-1, 100), Facing::Right))
        );

        // fully inside, not touching
        assert_eq!(
            Contained(Inside),
            s.bisect(&edge(Line::Diagonal(-1, 11), Facing::Left))
        );
        assert_eq!(
            Contained(Inside),
            s.bisect(&edge(Line::Diagonal(-1, -1), Facing::Right))
        );
        assert_eq!(
            Contained(Inside),
            s.bisect(&edge(Line::Diagonal(1, -1), Facing::Left))
        );
        assert_eq!(
            Contained(Inside),
            s.bisect(&edge(Line::Diagonal(1, 1), Facing::Right))
        );
        assert_eq!(
            Contained(Inside),
            s.bisect(&edge(Line::Diagonal(-1, 100), Facing::Left))
        );

        // fully inside, edge cases
        assert_eq!(
            Contained(Inside),
            s.bisect(&edge(Line::Diagonal(-1, 10), Facing::Left))
        );
        assert_eq!(
            Contained(Inside),
            s.bisect(&edge(Line::Diagonal(-1, 0), Facing::Right))
        );

        // for a flipped segment, it should return the same bisection results,
        // but with the resulting segment order and point orders flipped as well.
        let sr = segment(point(5, 5), point(0, 0));
        assert_eq!(
            Bisection(
                (segment(point(5, 5), point(2, 2)), InEdge::Inside),
                (segment(point(1, 1), point(0, 0)), InEdge::Outside),
            ),
            sr.bisect(&edge(Line::Diagonal(-1, 4), Facing::Right))
        );
        assert_eq!(
            Bisection(
                (segment(point(5, 5), point(3, 3)), InEdge::Inside),
                (segment(point(2, 2), point(0, 0)), InEdge::Outside),
            ),
            sr.bisect(&edge(Line::Diagonal(-1, 5), Facing::Right))
        );

        // vertical segment
        let sv = segment(point(3, 0), point(3, 5));
        assert_eq!(
            Bisection(
                (segment(point(3, 0), point(3, 2)), InEdge::Outside),
                (segment(point(3, 3), point(3, 5)), InEdge::Inside),
            ),
            sv.bisect(&edge(Line::Diagonal(1, 0), Facing::Left))
        );
        assert_eq!(
            Bisection(
                (segment(point(3, 0), point(3, 3)), InEdge::Inside),
                (segment(point(3, 4), point(3, 5)), InEdge::Outside),
            ),
            sv.bisect(&edge(Line::Diagonal(1, 0), Facing::Right))
        );
        assert_eq!(
            Contained(Inside),
            sv.bisect(&edge(Line::Diagonal(-1, 8), Facing::Left))
        );
        assert_eq!(
            Bisection(
                (segment(point(3, 0), point(3, 4)), InEdge::Outside),
                (segment(point(3, 5), point(3, 5)), InEdge::Inside),
            ),
            sv.bisect(&edge(Line::Diagonal(-1, 8), Facing::Right))
        );
        assert_eq!(
            Contained(Inside),
            sv.bisect(&edge(Line::Diagonal(-1, 20), Facing::Left))
        );
        assert_eq!(
            Contained(Outside),
            sv.bisect(&edge(Line::Diagonal(-1, 20), Facing::Right))
        );

        // horizontal segment
        let sh = segment(point(0, 3), point(5, 3));
        assert_eq!(
            Bisection(
                (segment(point(0, 3), point(2, 3)), InEdge::Outside),
                (segment(point(3, 3), point(5, 3)), InEdge::Inside),
            ),
            sh.bisect(&edge(Line::Diagonal(1, 0), Facing::Right))
        );
        assert_eq!(
            Bisection(
                (segment(point(0, 3), point(3, 3)), InEdge::Inside),
                (segment(point(4, 3), point(5, 3)), InEdge::Outside),
            ),
            sh.bisect(&edge(Line::Diagonal(1, 0), Facing::Left))
        );
        assert_eq!(
            Contained(Inside),
            sv.bisect(&edge(Line::Diagonal(-1, 20), Facing::Left))
        );
        assert_eq!(
            Contained(Outside),
            sv.bisect(&edge(Line::Diagonal(-1, 20), Facing::Right))
        );
    }

    #[test]
    fn test_polygon_segments() {
        assert_eq!(
            polygon(vec![point(0, 0), point(5, 0), point(5, 5), point(0, 5)]).segments(),
            vec![
                segment(point(0, 0), point(5, 0)),
                segment(point(5, 0), point(5, 5)),
                segment(point(5, 5), point(0, 5)),
                segment(point(0, 5), point(0, 0)),
            ]
        );

        assert_eq!(
            polygon(vec![point(0, 0), point(5, 0), point(5, 5)]).segments(),
            vec![
                segment(point(0, 0), point(5, 0)),
                segment(point(5, 0), point(5, 5)),
                segment(point(5, 5), point(0, 0)),
            ]
        );
    }

    #[test]
    fn test_polygon_bisect() {
        let square = polygon(vec![point(0, 0), point(0, 5), point(5, 5), point(5, 0)]);

        // cut off top right corner
        assert_eq!(
            square.bisect(&edge(Line::Diagonal(-1, 8), Facing::Right)),
            (
                Some(polygon(vec![
                    point(5, 2),
                    point(5, 0),
                    point(0, 0),
                    point(0, 5),
                    point(2, 5),
                ])),
                Some(polygon(vec![point(3, 5), point(5, 5), point(5, 3)])),
            )
        );

        // keep top right corner
        assert_eq!(
            square.bisect(&edge(Line::Diagonal(-1, 8), Facing::Left)),
            (
                Some(polygon(vec![point(4, 5), point(5, 5), point(5, 4)])),
                Some(polygon(vec![
                    point(5, 3),
                    point(5, 0),
                    point(0, 0),
                    point(0, 5),
                    point(3, 5),
                ])),
            )
        );

        // cut off bottom right corner
        assert_eq!(
            square.bisect(&edge(Line::Diagonal(1, -3), Facing::Right)),
            (
                Some(polygon(vec![
                    point(2, 0),
                    point(0, 0),
                    point(0, 5),
                    point(5, 5),
                    point(5, 3),
                ])),
                Some(polygon(vec![point(5, 2), point(5, 0), point(3, 0)])),
            )
        );

        // keep bottom right corner
        assert_eq!(
            square.bisect(&edge(Line::Diagonal(1, -3), Facing::Left)),
            (
                Some(polygon(vec![point(5, 1), point(5, 0), point(4, 0)])),
                Some(polygon(vec![
                    point(3, 0),
                    point(0, 0),
                    point(0, 5),
                    point(5, 5),
                    point(5, 2),
                ])),
            )
        );

        // cut down the middle
        assert_eq!(
            square.bisect(&edge(Line::Diagonal(-1, 5), Facing::Left)),
            (
                Some(polygon(vec![point(1, 5), point(5, 5), point(5, 1)])),
                Some(polygon(vec![point(5, 0), point(0, 0), point(0, 5)])),
            )
        );

        // polygon fully outside edge
        assert_eq!(
            square.bisect(&edge(Line::Diagonal(-1, 100), Facing::Right)),
            (Some(square.clone()), None)
        );

        // polygon fully inside edge
        assert_eq!(
            square.bisect(&edge(Line::Diagonal(-1, 100), Facing::Left)),
            (None, Some(square.clone()))
        );
    }

    #[test]
    fn test_polygon_bisect2() {
        // bisect a triangle
        let triangle = polygon(vec![point(0, 0), point(5, 5), point(5, 0)]);

        // y-intercept = 0
        // intersect at 0,0
        assert_eq!(
            triangle.bisect(&edge(Line::Diagonal(-1, 0), Facing::Left)),
            (
                Some(polygon(vec![
                    point(1, 1),
                    point(5, 5),
                    point(5, 0),
                    point(1, 0),
                ])),
                Some(polygon(vec![point(0, 0)])),
            )
        );

        // y-intercept = 1
        // intersect at 0.5,0.5
        assert_eq!(
            triangle.bisect(&edge(Line::Diagonal(-1, 1), Facing::Left)),
            (
                Some(polygon(vec![
                    point(1, 1),
                    point(5, 5),
                    point(5, 0),
                    point(2, 0),
                ])),
                Some(polygon(vec![point(1, 0), point(0, 0)])),
            )
        );

        // y-intercept = 2
        // intersect at 1,1
        assert_eq!(
            triangle.bisect(&edge(Line::Diagonal(-1, 2), Facing::Left)),
            (
                Some(polygon(vec![
                    point(2, 2),
                    point(5, 5),
                    point(5, 0),
                    point(3, 0),
                    point(2, 1)
                ])),
                Some(polygon(vec![point(2, 0), point(0, 0), point(1, 1)])),
            )
        );

        // y-intercept = 3
        // intersect at 1.5,1.5
        assert_eq!(
            triangle.bisect(&edge(Line::Diagonal(-1, 3), Facing::Left)),
            (
                Some(polygon(vec![
                    point(2, 2),
                    point(5, 5),
                    point(5, 0),
                    point(4, 0),
                ])),
                Some(polygon(vec![
                    point(3, 0),
                    point(0, 0),
                    point(1, 1),
                    point(2, 1),
                ])),
            )
        );

        // y-intercept = 4
        // intersect at 2,2
        assert_eq!(
            triangle.bisect(&edge(Line::Diagonal(-1, 4), Facing::Left)),
            (
                Some(polygon(vec![
                    point(3, 3),
                    point(5, 5),
                    point(5, 0),
                    point(3, 2),
                ])),
                Some(polygon(vec![point(4, 0), point(0, 0), point(2, 2)])),
            )
        );
    }
}
