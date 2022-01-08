use std::cmp;
use std::fmt;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INSTRUCTION_RE: Regex = Regex::new(
        r"\A(on|off) x=(\-?\d+)\.\.(\-?\d+),y=(\-?\d+)\.\.(\-?\d+),z=(\-?\d+)\.\.(\-?\d+)\z"
    )
    .unwrap();
}

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[derive(Debug)]
struct RebootProcedure {
    instructions: Vec<Instruction>,
    state: ReactorState,
}

impl RebootProcedure {
    fn parse(input: &str) -> RebootProcedure {
        let instructions = input.lines().map(|line| Instruction::parse(line)).collect();
        let state = ReactorState::new();
        RebootProcedure {
            instructions,
            state,
        }
    }

    fn execute(&mut self) {
        for instruction in &self.instructions {
            println!("applying {}", instruction,);
            self.state = self.state.apply(instruction);
        }
    }
}

#[derive(Debug)]
struct Instruction {
    on: bool,
    area: Cuboid,
}

impl Instruction {
    fn parse(input: &str) -> Instruction {
        let captures = INSTRUCTION_RE.captures(input).unwrap();
        let on_off = captures.get(1).unwrap().as_str();
        let x_min = captures.get(2).unwrap().as_str().parse::<isize>().unwrap();
        let x_max = captures.get(3).unwrap().as_str().parse::<isize>().unwrap();
        let y_min = captures.get(4).unwrap().as_str().parse::<isize>().unwrap();
        let y_max = captures.get(5).unwrap().as_str().parse::<isize>().unwrap();
        let z_min = captures.get(6).unwrap().as_str().parse::<isize>().unwrap();
        let z_max = captures.get(7).unwrap().as_str().parse::<isize>().unwrap();
        let x = InclusiveRange::new(x_min, x_max);
        let y = InclusiveRange::new(y_min, y_max);
        let z = InclusiveRange::new(z_min, z_max);
        let area = Cuboid::new(x, y, z);
        let on = match on_off {
            "on" => true,
            "off" => false,
            _ => panic!("Unreachable"),
        };
        Instruction { on, area }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let on_off = if self.on { "on" } else { "off" };
        write!(f, "{} {}", on_off, self.area)
    }
}

#[derive(Clone, Copy, Debug)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Debug)]
struct InclusiveRange {
    min: isize,
    max: isize,
}

impl InclusiveRange {
    fn new(min: isize, max: isize) -> InclusiveRange {
        assert!(min <= max);
        InclusiveRange { min, max }
    }

    fn overlaps(&self, other: &InclusiveRange) -> bool {
        self.contains(other.min)
            || self.contains(other.max)
            || other.contains(self.min)
            || other.contains(self.max)
    }

    fn contains(&self, coord: isize) -> bool {
        coord >= self.min && coord <= self.max
    }

    fn subset_of(&self, other: &InclusiveRange) -> bool {
        self.min >= other.min && self.max <= other.max
    }

    fn maybe_bisect(&self, coord: isize) -> Option<(InclusiveRange, InclusiveRange)> {
        if coord > self.min && coord <= self.max {
            let left = InclusiveRange::new(self.min, coord - 1);
            let right = InclusiveRange::new(coord, self.max);
            Some((left, right))
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        (self.max - self.min + 1) as usize
    }

    fn count_within(&self, target: &InclusiveRange) -> usize {
        let min = cmp::max(self.min, target.min);
        let max = cmp::min(self.max, target.max);
        if max >= min {
            (max - min + 1) as usize
        } else {
            0
        }
    }
}

impl fmt::Display for InclusiveRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.min, self.max)
    }
}

#[derive(Clone, Debug)]
struct Cuboid {
    x: InclusiveRange,
    y: InclusiveRange,
    z: InclusiveRange,
}

impl Cuboid {
    fn new(x: InclusiveRange, y: InclusiveRange, z: InclusiveRange) -> Cuboid {
        Cuboid { x, y, z }
    }

    fn len(&self) -> usize {
        self.x.len() * self.y.len() * self.z.len()
    }

    fn count_within(&self, area: &Cuboid) -> usize {
        self.x.count_within(&area.x) * self.y.count_within(&area.y) * self.z.count_within(&area.z)
    }

    fn overlaps(&self, area: &Cuboid) -> bool {
        self.x.overlaps(&area.x) && self.y.overlaps(&area.y) && self.z.overlaps(&area.z)
    }

    fn subset_of(&self, area: &Cuboid) -> bool {
        self.x.subset_of(&area.x) && self.y.subset_of(&area.y) && self.z.subset_of(&area.z)
    }

    fn bisect_all(cuboids: Vec<Cuboid>, axis: Axis, coord: isize) -> Vec<Cuboid> {
        let mut output = vec![];
        for cuboid in cuboids {
            let (first, maybe_second) = cuboid.bisect(axis, coord);
            output.push(first);
            if maybe_second.is_some() {
                output.push(maybe_second.unwrap());
            }
        }
        output
    }

    fn bisect(self, axis: Axis, coord: isize) -> (Cuboid, Option<Cuboid>) {
        match axis {
            Axis::X => match self.x.maybe_bisect(coord) {
                Some((first_x, second_x)) => {
                    let first = Cuboid::new(first_x, self.y, self.z);
                    let second = Cuboid::new(second_x, self.y, self.z);
                    (first, Some(second))
                }
                None => (self, None),
            },
            Axis::Y => match self.y.maybe_bisect(coord) {
                Some((first_y, second_y)) => {
                    let first = Cuboid::new(self.x, first_y, self.z);
                    let second = Cuboid::new(self.x, second_y, self.z);
                    (first, Some(second))
                }
                None => (self, None),
            },
            Axis::Z => match self.z.maybe_bisect(coord) {
                Some((first_z, second_z)) => {
                    let first = Cuboid::new(self.x, self.y, first_z);
                    let second = Cuboid::new(self.x, self.y, second_z);
                    (first, Some(second))
                }
                None => (self, None),
            },
        }
    }
}

impl fmt::Display for Cuboid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x={},y={},z={} ({})", self.x, self.y, self.z, self.len())
    }
}

#[derive(Debug)]
struct ReactorState {
    on_regions: Vec<Cuboid>,
}

impl ReactorState {
    fn new() -> ReactorState {
        ReactorState { on_regions: vec![] }
    }

    fn apply(&self, instruction: &Instruction) -> ReactorState {
        if instruction.on {
            self.apply_on(&instruction.area)
        } else {
            self.apply_off(&instruction.area)
        }
    }

    fn apply_on(&self, area: &Cuboid) -> ReactorState {
        // first, remove any overlaps, just as if this was an off instruction
        let mut new_on_regions = self.remove_region(area);

        // now, add the new area to the set
        new_on_regions.push(area.clone());

        ReactorState {
            on_regions: new_on_regions,
        }
    }

    fn apply_off(&self, area: &Cuboid) -> ReactorState {
        ReactorState {
            on_regions: self.remove_region(area),
        }
    }

    fn remove_region(&self, area: &Cuboid) -> Vec<Cuboid> {
        // bisect along all planes of the area, to avoid any messy overlap
        let mut non_overlapping = vec![];
        let mut working_set =
            Self::filter_overlapping(self.on_regions.clone(), &mut non_overlapping, &area);

        working_set = Cuboid::bisect_all(working_set, Axis::X, area.x.min);
        working_set = Self::filter_overlapping(working_set, &mut non_overlapping, &area);

        working_set = Cuboid::bisect_all(working_set, Axis::X, area.x.max + 1);
        working_set = Self::filter_overlapping(working_set, &mut non_overlapping, &area);

        working_set = Cuboid::bisect_all(working_set, Axis::Y, area.y.min);
        working_set = Self::filter_overlapping(working_set, &mut non_overlapping, &area);

        working_set = Cuboid::bisect_all(working_set, Axis::Y, area.y.max + 1);
        working_set = Self::filter_overlapping(working_set, &mut non_overlapping, &area);

        working_set = Cuboid::bisect_all(working_set, Axis::Z, area.z.min);
        working_set = Self::filter_overlapping(working_set, &mut non_overlapping, &area);

        working_set = Cuboid::bisect_all(working_set, Axis::Z, area.z.max + 1);
        working_set = Self::filter_overlapping(working_set, &mut non_overlapping, &area);

        // ensure all remaining cuboids in working set should be removed
        assert!(working_set.into_iter().all(|c| c.subset_of(area)));

        non_overlapping
    }

    fn filter_overlapping(
        working_set: Vec<Cuboid>,
        non_overlapping: &mut Vec<Cuboid>,
        area: &Cuboid,
    ) -> Vec<Cuboid> {
        let mut result = vec![];
        for c in working_set {
            if c.overlaps(area) {
                result.push(c);
            } else {
                non_overlapping.push(c);
            }
        }
        result
    }

    fn count(&self) -> usize {
        self.on_regions.iter().map(|c| c.len()).sum()
    }

    fn count_within(&self, area: &Cuboid) -> usize {
        self.on_regions.iter().map(|c| c.count_within(area)).sum()
    }
}

impl fmt::Display for ReactorState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for region in &self.on_regions {
            write!(f, "\n  {}", region)?;
        }
        Ok(())
    }
}

fn part1(input: &str) -> usize {
    let mut reboot = RebootProcedure::parse(input);
    reboot.execute();
    let range = InclusiveRange::new(-50, 50);
    let count_area = Cuboid::new(range, range, range);
    reboot.state.count_within(&count_area)
}

fn part2(input: &str) -> usize {
    let mut reboot = RebootProcedure::parse(input);
    reboot.execute();
    reboot.state.count()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        on x=10..12,y=10..12,z=10..12
        on x=11..13,y=11..13,z=11..13
        off x=9..11,y=9..11,z=9..11
        on x=10..10,y=10..10,z=10..10
    "};

    static EXAMPLE2: &str = indoc! {"
        on x=-20..26,y=-36..17,z=-47..7
        on x=-20..33,y=-21..23,z=-26..28
        on x=-22..28,y=-29..23,z=-38..16
        on x=-46..7,y=-6..46,z=-50..-1
        on x=-49..1,y=-3..46,z=-24..28
        on x=2..47,y=-22..22,z=-23..27
        on x=-27..23,y=-28..26,z=-21..29
        on x=-39..5,y=-6..47,z=-3..44
        on x=-30..21,y=-8..43,z=-13..34
        on x=-22..26,y=-27..20,z=-29..19
        off x=-48..-32,y=26..41,z=-47..-37
        on x=-12..35,y=6..50,z=-50..-2
        off x=-48..-32,y=-32..-16,z=-15..-5
        on x=-18..26,y=-33..15,z=-7..46
        off x=-40..-22,y=-38..-28,z=23..41
        on x=-16..35,y=-41..10,z=-47..6
        off x=-32..-23,y=11..30,z=-14..3
        on x=-49..-5,y=-3..45,z=-29..18
        off x=18..30,y=-20..-8,z=-3..13
        on x=-41..9,y=-7..43,z=-33..15
        on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
        on x=967..23432,y=45373..81175,z=27513..53682
    "};

    static EXAMPLE3: &str = indoc! {"
        on x=-5..47,y=-31..22,z=-19..33
        on x=-44..5,y=-27..21,z=-14..35
        on x=-49..-1,y=-11..42,z=-10..38
        on x=-20..34,y=-40..6,z=-44..1
        off x=26..39,y=40..50,z=-2..11
        on x=-41..5,y=-41..6,z=-36..8
        off x=-43..-33,y=-45..-28,z=7..25
        on x=-33..15,y=-32..19,z=-34..11
        off x=35..47,y=-46..-34,z=-11..5
        on x=-14..36,y=-6..44,z=-16..29
        on x=-57795..-6158,y=29564..72030,z=20435..90618
        on x=36731..105352,y=-21140..28532,z=16094..90401
        on x=30999..107136,y=-53464..15513,z=8553..71215
        on x=13528..83982,y=-99403..-27377,z=-24141..23996
        on x=-72682..-12347,y=18159..111354,z=7391..80950
        on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
        on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
        on x=-52752..22273,y=-49450..9096,z=54442..119054
        on x=-29982..40483,y=-108474..-28371,z=-24328..38471
        on x=-4958..62750,y=40422..118853,z=-7672..65583
        on x=55694..108686,y=-43367..46958,z=-26781..48729
        on x=-98497..-18186,y=-63569..3412,z=1232..88485
        on x=-726..56291,y=-62629..13224,z=18033..85226
        on x=-110886..-34664,y=-81338..-8658,z=8914..63723
        on x=-55829..24974,y=-16897..54165,z=-121762..-28058
        on x=-65152..-11147,y=22489..91432,z=-58782..1780
        on x=-120100..-32970,y=-46592..27473,z=-11695..61039
        on x=-18631..37533,y=-124565..-50804,z=-35667..28308
        on x=-57817..18248,y=49321..117703,z=5745..55881
        on x=14781..98692,y=-1341..70827,z=15753..70151
        on x=-34419..55919,y=-19626..40991,z=39015..114138
        on x=-60785..11593,y=-56135..2999,z=-95368..-26915
        on x=-32178..58085,y=17647..101866,z=-91405..-8878
        on x=-53655..12091,y=50097..105568,z=-75335..-4862
        on x=-111166..-40997,y=-71714..2688,z=5609..50954
        on x=-16602..70118,y=-98693..-44401,z=5197..76897
        on x=16383..101554,y=4615..83635,z=-44907..18747
        off x=-95822..-15171,y=-19987..48940,z=10804..104439
        on x=-89813..-14614,y=16069..88491,z=-3297..45228
        on x=41075..99376,y=-20427..49978,z=-52012..13762
        on x=-21330..50085,y=-17944..62733,z=-112280..-30197
        on x=-16478..35915,y=36008..118594,z=-7885..47086
        off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
        off x=2032..69770,y=-71013..4824,z=7471..94418
        on x=43670..120875,y=-42068..12382,z=-24787..38892
        off x=37514..111226,y=-45862..25743,z=-16714..54663
        off x=25699..97951,y=-30668..59918,z=-15349..69697
        off x=-44271..17935,y=-9516..60759,z=49131..112598
        on x=-61695..-5813,y=40978..94975,z=8655..80240
        off x=-101086..-9439,y=-7088..67543,z=33935..83858
        off x=18020..114017,y=-48931..32606,z=21474..89843
        off x=-77139..10506,y=-89994..-18797,z=-80..59318
        off x=8476..79288,y=-75520..11602,z=-96624..-24783
        on x=-47488..-1262,y=24338..100707,z=16292..72967
        off x=-84341..13987,y=2429..92914,z=-90671..-1318
        off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
        off x=-27365..46395,y=31009..98017,z=15428..76570
        off x=-70369..-16548,y=22648..78696,z=-1892..86821
        on x=-53470..21291,y=-120233..-33476,z=-44150..38147
        off x=-93533..-4276,y=-16170..68771,z=-104985..-24507
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 39);
    }

    #[test]
    fn test_part1_example2() {
        let result = part1(EXAMPLE2);
        assert_eq!(result, 590784);
    }

    #[test]
    fn test_part1_example3() {
        let result = part1(EXAMPLE3);
        assert_eq!(result, 474140);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 612714);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(EXAMPLE1);
        assert_eq!(result, 39);
    }

    #[test]
    fn test_part2_example2() {
        let result = part2(EXAMPLE2);
        assert_eq!(result, 39769202357779);
    }

    #[test]
    fn test_part2_example3() {
        let result = part2(EXAMPLE3);
        assert_eq!(result, 2758514936282235);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 1311612259117092);
    }
}
