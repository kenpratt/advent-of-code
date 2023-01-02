pub mod astar;

use astar::AStarInterface;

use std::cmp;
use std::fmt;
use std::fs;

use std::ops::{Add, Mul, Sub};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

const MAX_MINUTES: usize = 32;

#[derive(Debug)]
struct Blueprint {
    id: u16,
    ore: ResourceAmounts,
    clay: ResourceAmounts,
    obsidian: ResourceAmounts,
    geode: ResourceAmounts,
}

impl Blueprint {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref BLUEPRINT_RE: Regex = Regex::new(r"\ABlueprint (\d+): Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each obsidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian\.\z").unwrap();
        }

        let caps = BLUEPRINT_RE.captures(input).unwrap();
        let nums: Vec<u16> = caps
            .iter()
            .skip(1)
            .map(|s| s.unwrap().as_str().parse::<u16>().unwrap())
            .collect();
        assert_eq!(nums.len(), 7);

        let id = nums[0];
        let ore = ResourceAmounts::new(nums[1], 0, 0, 0);
        let clay = ResourceAmounts::new(nums[2], 0, 0, 0);
        let obsidian = ResourceAmounts::new(nums[3], nums[4], 0, 0);
        let geode = ResourceAmounts::new(nums[5], 0, nums[6], 0);

        Self {
            id,
            ore,
            clay,
            obsidian,
            geode,
        }
    }

    fn costs(&self, resource: &Resource) -> ResourceAmounts {
        use Resource::*;
        match resource {
            Ore => self.ore,
            Clay => self.clay,
            Obsidian => self.obsidian,
            Geode => self.geode,
        }
    }

    fn build_time_before_end(&self, resource: &Resource) -> u16 {
        use Resource::*;
        match resource {
            // no point building an ore robot on the last 3 turns + the cost, as it won't
            // have time to collect ore in time to pay for itself & provide resources for a geode robot
            Ore => 3 + self.ore.ore,

            // no point building a clay robot on the last 5 turns, as it won't
            // have time to collect clay in time to build a ore/obsidian robot in
            // time to provide resources for a geode robot
            Clay => 5,

            // no point building an obsidian robot on the last 3 turns, as it
            // won't have time to collect obsidian in time to provide resources for a geode robot
            Obsidian => 3,

            // no point building a geode robot on the last turn, it won't have time
            // to collect geodes
            Geode => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Resource::*;
        match self {
            Ore => write!(f, "Ore"),
            Clay => write!(f, "Clay"),
            Obsidian => write!(f, "Obsidian"),
            Geode => write!(f, "Geode"),
        }
    }
}

const RESOURCES: [Resource; 4] = [
    Resource::Ore,
    Resource::Clay,
    Resource::Obsidian,
    Resource::Geode,
];

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct ResourceAmounts {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
}

impl ResourceAmounts {
    fn new(ore: u16, clay: u16, obsidian: u16, geode: u16) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }

    fn get(&self, resource: &Resource) -> u16 {
        use Resource::*;
        match resource {
            Ore => self.ore,
            Clay => self.clay,
            Obsidian => self.obsidian,
            Geode => self.geode,
        }
    }

    fn gte(&self, other: &ResourceAmounts) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }

    fn increase(&mut self, resource: &Resource, n: u16) {
        use Resource::*;
        match resource {
            Ore => self.ore += n,
            Clay => self.clay += n,
            Obsidian => self.obsidian += n,
            Geode => self.geode += n,
        }
    }
}

impl Add<ResourceAmounts> for ResourceAmounts {
    type Output = ResourceAmounts;

    fn add(self, other: ResourceAmounts) -> ResourceAmounts {
        ResourceAmounts {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}

impl Sub<ResourceAmounts> for ResourceAmounts {
    type Output = ResourceAmounts;

    fn sub(self, other: ResourceAmounts) -> ResourceAmounts {
        ResourceAmounts {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

impl Mul<u16> for ResourceAmounts {
    type Output = ResourceAmounts;

    fn mul(self, n: u16) -> ResourceAmounts {
        ResourceAmounts {
            ore: self.ore * n,
            clay: self.clay * n,
            obsidian: self.obsidian * n,
            geode: self.geode * n,
        }
    }
}

impl fmt::Display for ResourceAmounts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.ore, self.clay, self.obsidian, self.geode
        )
    }
}

// won't build anything in last minute
const BUILD_ARRAY_SIZE: usize = MAX_MINUTES - 1;

// used for heuristic/score
// set to an unrealistic number of geode robots
const GEODE_BENCHMARK: u16 = 20;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Build {
    minute: u16,
    bank: ResourceAmounts,
    income: ResourceAmounts,
    purchases: [Option<Resource>; BUILD_ARRAY_SIZE],
}

impl Build {
    fn blank() -> Self {
        Self {
            minute: 0,
            bank: ResourceAmounts::new(0, 0, 0, 0),
            income: ResourceAmounts::new(1, 0, 0, 0),
            purchases: [None; BUILD_ARRAY_SIZE],
        }
    }

    fn complete(&self, num_minutes: u16) -> bool {
        self.minute == num_minutes
    }

    fn score(&self) -> u16 {
        self.minute * GEODE_BENCHMARK - self.bank.geode
    }

    fn heuristic(&self, num_minutes: u16, blueprint: &Blueprint) -> u16 {
        if self.complete(num_minutes) {
            return 0;
        }

        // estimate future geode income (both guarunteed from existing geodes,
        // and hypothetical income from future builds - the latter should be an over-estimate)
        let estimate = self.estimate_final_state(num_minutes, blueprint);
        estimate.score() - self.score()
    }

    fn estimate_final_state(&self, num_minutes: u16, blueprint: &Blueprint) -> Self {
        let mut build = self.clone();
        while !build.complete(num_minutes) {
            if build.income.clay == 0 {
                // for clay, we can assume saving up is always the best choice,
                // as building geode robots will never accelerate a clay build
                // (the closest blueprint is ore=2, clay=4, but even in that
                // case, it is better to save up for a clay robot at minute 4,
                // as if you build ore at minute 2, still won't have 4 clay
                // until minute 5.
                build.estimate_build(&Resource::Clay, None, num_minutes, blueprint)
            } else if build.income.obsidian == 0 {
                // now that clay is built, optimistically assume we get free
                // ore ard clay robots every single minute.
                build.estimate_build(
                    &Resource::Obsidian,
                    Some(ResourceAmounts::new(1, 1, 0, 0)),
                    num_minutes,
                    blueprint,
                )
            } else {
                // now that clay is built, optimistically assume we get free
                // ore, clay and obsidian robots every single minute.
                build.estimate_build(
                    &Resource::Geode,
                    Some(ResourceAmounts::new(1, 1, 1, 0)),
                    num_minutes,
                    blueprint,
                )
            };
        }
        build
    }

    fn estimate_build(
        &mut self,
        robot: &Resource,
        auto_production_increase: Option<ResourceAmounts>,
        num_minutes: u16,
        blueprint: &Blueprint,
    ) {
        let mut built = false;
        while !built && !self.complete(num_minutes) {
            let cost = blueprint.costs(robot);
            let can_afford = self.bank.gte(&cost);

            // earn income (pre-builds)
            self.bank = self.bank + self.income;

            if can_afford {
                // build robot! (and pay for it)
                self.bank = self.bank - cost;
                self.income.increase(robot, 1);
                built = true;
            } else if auto_production_increase.is_some() {
                // apply a production increase heuristic to ensure we're being optimistic enough
                // these increases are free!
                self.income = self.income + auto_production_increase.unwrap();
            }

            self.minute += 1;
        }
    }

    fn neighbours(&self, num_minutes: u16, blueprint: &Blueprint) -> Vec<(Self, u16)> {
        let mut builds: Vec<Self> = RESOURCES
            .iter()
            .flat_map(|resource| self.build_next(resource, num_minutes, blueprint))
            .collect();

        if builds.is_empty() {
            // special case, we are finished this build branch as we have nothing else to build
            builds.push(self.finalize(num_minutes));
        }

        let self_score = self.score();
        builds
            .into_iter()
            .map(|b| {
                let b_score = b.score();
                let score = b_score - self_score;
                (b, score)
            })
            .collect()
    }

    fn build_next(
        &self,
        robot: &Resource,
        num_minutes: u16,
        blueprint: &Blueprint,
    ) -> Option<Self> {
        let affordable_at_minute = self.affordable_at(robot, blueprint);
        match affordable_at_minute {
            Some(minute) => {
                let last_minute_to_build = num_minutes - 1 - blueprint.build_time_before_end(robot);
                if minute <= last_minute_to_build {
                    Some(self.place_robot(robot, minute, blueprint))
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn place_robot(&self, robot: &Resource, minute: u16, blueprint: &Blueprint) -> Self {
        let mut purchases = self.purchases.clone();
        purchases[minute as usize] = Some(*robot);

        // add income from intermediate turns, minus the cost of the robot
        let bank = self.advance_bank(minute + 1 - self.minute) - blueprint.costs(robot);

        // add robot to income
        let mut income = self.income.clone();
        income.increase(robot, 1);

        Build {
            minute: minute + 1,
            bank: bank,
            income: income,
            purchases: purchases,
        }
    }

    fn affordable_at(&self, robot: &Resource, blueprint: &Blueprint) -> Option<u16> {
        Self::time_to_save_for_robot(robot, &self.bank, &self.income, blueprint)
            .map(|m| self.minute + m)
    }

    fn time_to_save_for_robot(
        robot: &Resource,
        bank: &ResourceAmounts,
        income: &ResourceAmounts,
        blueprint: &Blueprint,
    ) -> Option<u16> {
        let costs = blueprint.costs(robot);
        RESOURCES
            .iter()
            .map(|resource| {
                let cost = costs.get(resource);
                Self::time_to_save_for_resource(resource, cost, bank, income)
            })
            .reduce(|acc, val| {
                if acc.is_none() || val.is_none() {
                    None
                } else {
                    Some(cmp::max(acc.unwrap(), val.unwrap()))
                }
            })
            .unwrap()
    }

    fn time_to_save_for_resource(
        resource: &Resource,
        needed: u16,
        bank: &ResourceAmounts,
        income: &ResourceAmounts,
    ) -> Option<u16> {
        let amount = bank.get(resource);
        let income = income.get(resource);

        if amount >= needed {
            return Some(0);
        } else if income == 0 {
            return None;
        } else {
            let shortfall = needed - amount;

            let turns = if shortfall % income == 0 {
                shortfall / income
            } else {
                shortfall / income + 1
            };

            Some(turns)
        }
    }

    fn advance_bank(&self, minutes: u16) -> ResourceAmounts {
        self.bank + self.income * minutes
    }

    fn finalize(&self, num_minutes: u16) -> Self {
        let mut build = self.clone();
        build.minute = num_minutes;
        build.bank = self.advance_bank(num_minutes - self.minute);
        build
    }
}

#[derive(Debug)]
struct Solver<'a> {
    num_minutes: u16,
    blueprint: &'a Blueprint,
    builds: Vec<Build>,
}

impl Solver<'_> {
    fn run(num_minutes: u16, blueprint: &Blueprint) -> u16 {
        if num_minutes as usize > MAX_MINUTES {
            panic!("Minutes exceeds hard-coded max");
        }

        let mut solver = Solver {
            num_minutes,
            blueprint,
            builds: vec![],
        };
        let initial_build = solver.add(Build::blank());

        match solver.shortest_path(initial_build, true) {
            Some((path, _length)) => {
                let best_id = &path.last().unwrap().0;
                let best_build = solver.get(best_id);
                best_build.bank.geode
            }
            None => 0, // no solution found
        }
    }

    fn add(&mut self, build: Build) -> usize {
        self.builds.push(build);
        self.builds.len() - 1
    }

    fn get(&self, id: &usize) -> &Build {
        &self.builds[*id]
    }
}

impl AStarInterface<usize> for Solver<'_> {
    fn at_goal(&self, node: &usize) -> bool {
        let build = self.get(node);
        build.complete(self.num_minutes)
    }

    fn heuristic(&self, from: &usize) -> u16 {
        let build = self.get(from);
        build.heuristic(self.num_minutes, &self.blueprint)
    }

    fn neighbours(&mut self, from: &usize) -> Vec<(usize, u16)> {
        let build = self.get(from);
        build
            .neighbours(self.num_minutes, &self.blueprint)
            .into_iter()
            .map(|(n, score)| (self.add(n), score))
            .collect()
    }
}

fn maximize_geodes(blueprint: &Blueprint, num_minutes: u16) -> u16 {
    Solver::run(num_minutes, blueprint)
}

fn part1(input: &str) -> usize {
    let blueprints = Blueprint::parse_list(input);
    blueprints
        .iter()
        .map(|b| b.id as usize * maximize_geodes(b, 24) as usize)
        .sum()
}

fn part2(input: &str) -> usize {
    let blueprints = Blueprint::parse_list(input);
    blueprints[0..3]
        .iter()
        .map(|b| maximize_geodes(b, 32) as usize)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE: &str = indoc! {"
        Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
    "};

    #[test]
    fn test_part1_blueprint1() {
        let blueprints = Blueprint::parse_list(EXAMPLE);
        let result = maximize_geodes(&blueprints[0], 24);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_part1_blueprint2() {
        let blueprints = Blueprint::parse_list(EXAMPLE);
        let result = maximize_geodes(&blueprints[1], 24);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 33);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 1616);
    }

    #[test]
    fn test_part2_blueprint1() {
        let blueprints = Blueprint::parse_list(EXAMPLE);
        let result = maximize_geodes(&blueprints[0], 32);
        assert_eq!(result, 56);
    }

    #[test]
    fn test_part2_blueprint2() {
        let blueprints = Blueprint::parse_list(EXAMPLE);
        let result = maximize_geodes(&blueprints[1], 32);
        assert_eq!(result, 62);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file());
        assert_eq!(result, 8990);
    }
}
