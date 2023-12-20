use std::{
    collections::{hash_map::DefaultHasher, BTreeMap, HashMap, VecDeque},
    fs,
    hash::{Hash, Hasher},
};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    // println!("part 2 result: {:?}", part2(&read_input_file()));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

const BROADCAST_NAME: &'static str = "broadcaster";
const BUTTON_NAME: &'static str = "button";
const OUTPUT_NAMES: [&'static str; 2] = ["output", "rx"];

#[derive(Debug)]
struct Configuration {
    modules: HashMap<String, Module>,
    module_state: ModuleState,
}

impl Configuration {
    fn parse(input: &str) -> Self {
        let modules = Module::parse_list(input)
            .into_iter()
            .map(|m| (m.name.clone(), m))
            .collect();
        let module_state = ModuleState::new(&modules);
        Self {
            modules,
            module_state,
        }
    }

    fn count_pulses_for_presses(&mut self, times: usize) -> usize {
        let mut module_state_history = HashMap::new();
        module_state_history.insert(calculate_hash(&self.module_state), 0);

        let mut pulse_count_history = vec![(0, 0)];

        // find a repeated state
        let mut num_presses = 0;
        while num_presses < times {
            let pulse_counts = self.push_button();
            num_presses += 1;
            pulse_count_history.push(pulse_counts);

            match module_state_history.get(&calculate_hash(&self.module_state)) {
                Some(prev_presses) => {
                    let pulse_counts_for_loop =
                        &pulse_count_history[(prev_presses + 1)..=num_presses];
                    return Self::count_pulses(times, pulse_counts_for_loop);
                }
                None => {
                    module_state_history.insert(calculate_hash(&self.module_state), num_presses);
                }
            }
        }

        Self::count_pulses(times, &pulse_count_history[1..])
    }

    fn count_pulses(times: usize, pulse_counts: &[(usize, usize)]) -> usize {
        let loop_size = pulse_counts.len();
        let num_loops = times / loop_size;
        let rem = times % loop_size;
        if rem != 0 {
            panic!("Haven't implemented remainder yet: {} {}", times, loop_size);
        }

        let num_high: usize = pulse_counts.iter().map(|(high, _low)| high).sum();
        let num_low: usize = pulse_counts.iter().map(|(_high, low)| low).sum();

        (num_high * num_loops) * (num_low * num_loops)
    }

    fn push_button(&mut self) -> (usize, usize) {
        let mut pulses: VecDeque<(Pulse, String, String)> = VecDeque::new();
        pulses.push_back((
            Pulse::Low,
            BUTTON_NAME.to_string(),
            BROADCAST_NAME.to_string(),
        ));

        let mut num_high = 0;
        let mut num_low = 0;

        while let Some((pulse, source, destination)) = pulses.pop_front() {
            // println!("{} -{:?}-> {}", source, pulse, destination);

            match pulse {
                Pulse::High => num_high += 1,
                Pulse::Low => num_low += 1,
            }

            match self.modules.get(&destination) {
                Some(module) => {
                    let new_pulses = module.process(pulse, source, &mut self.module_state);
                    for p in new_pulses {
                        pulses.push_back(p);
                    }
                }
                None => {
                    // output destination, do nothing
                    assert_eq!(OUTPUT_NAMES.contains(&destination.as_str()), true);
                }
            }
        }

        (num_high, num_low)
    }
}

#[derive(Debug)]
struct Module {
    name: String,
    kind: ModuleType,
    destinations: Vec<String>,
}

impl Module {
    fn parse_list(input: &str) -> Vec<Self> {
        input.lines().map(|line| Self::parse(line)).collect()
    }

    fn parse(input: &str) -> Self {
        lazy_static! {
            static ref MODULE_RE: Regex =
                Regex::new(r"\A([%&]?)([a-z]+) \-> ([a-z ,]+)\z").unwrap();
        }

        let caps = MODULE_RE.captures(input).unwrap();
        let kind = ModuleType::parse(caps.get(1).unwrap().as_str());
        let name = caps.get(2).unwrap().as_str().to_string();
        let destinations = caps
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.to_string())
            .collect();

        if kind == ModuleType::Broadcast {
            assert_eq!(name, BROADCAST_NAME);
        }

        Self {
            name,
            kind,
            destinations,
        }
    }

    fn process<'b>(
        &self,
        pulse: Pulse,
        source: String,
        module_state: &'b mut ModuleState,
    ) -> Vec<(Pulse, String, String)> {
        let output_pulse: Option<Pulse> = match self.kind {
            ModuleType::Broadcast => Some(pulse),
            ModuleType::Conjunction => {
                // update memory
                let memory = module_state.get_conjunction(&self.name);
                memory.insert(source, pulse);

                // check if all inputs are high
                if memory.iter().all(|(_s, p)| *p == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            ModuleType::FlipFlop => {
                match pulse {
                    Pulse::High => None, // ignore
                    Pulse::Low => {
                        // activate flip-flop
                        let state = module_state.get_flip_flop(&self.name);
                        *state = !*state;
                        match *state {
                            true => Some(Pulse::High),
                            false => Some(Pulse::Low),
                        }
                    }
                }
            }
        };

        match output_pulse {
            Some(p) => self
                .destinations
                .iter()
                .map(|dest| (p, self.name.clone(), dest.clone()))
                .collect(),
            None => vec![],
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum ModuleType {
    Broadcast,
    Conjunction,
    FlipFlop,
}

impl ModuleType {
    fn parse(input: &str) -> Self {
        use ModuleType::*;

        match input {
            "%" => FlipFlop,
            "&" => Conjunction,
            "" => Broadcast,
            _ => panic!("Unexpected module type: {:?}", input),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct ModuleState {
    conjunctions: BTreeMap<String, BTreeMap<String, Pulse>>,
    flip_flops: BTreeMap<String, bool>,
}

impl ModuleState {
    fn new(modules: &HashMap<String, Module>) -> Self {
        // initialize as maps of inbound connections to low pulse
        let conjunctions: BTreeMap<String, BTreeMap<String, Pulse>> = modules
            .iter()
            .filter(|(_, module)| module.kind == ModuleType::Conjunction)
            .map(|(_, module)| {
                let memory: BTreeMap<String, Pulse> = modules
                    .iter()
                    .filter(|(_, other)| other.destinations.contains(&module.name))
                    .map(|(n, _)| (n.clone(), Pulse::Low))
                    .collect();
                (module.name.clone(), memory)
            })
            .collect();

        // initialize as false
        let flip_flops: BTreeMap<String, bool> = modules
            .iter()
            .filter(|(_, module)| module.kind == ModuleType::FlipFlop)
            .map(|(_, module)| (module.name.clone(), false))
            .collect();

        ModuleState {
            conjunctions,
            flip_flops,
        }
    }

    fn get_conjunction(&mut self, module: &String) -> &mut BTreeMap<String, Pulse> {
        self.conjunctions.get_mut(module).unwrap()
    }

    fn get_flip_flop(&mut self, module: &String) -> &mut bool {
        self.flip_flops.get_mut(module).unwrap()
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn part1(input: &str) -> usize {
    let mut config = Configuration::parse(input);
    config.count_pulses_for_presses(1000)
}

// fn part2(input: &str) -> usize {
//     let items = Data::parse(input);
//     dbg!(&items);
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    static EXAMPLE1: &str = indoc! {"
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a
    "};

    static EXAMPLE2: &str = indoc! {"
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output
    "};

    #[test]
    fn test_part1_example1() {
        let result = part1(EXAMPLE1);
        assert_eq!(result, 32000000);
    }

    #[test]
    fn test_part1_example2() {
        let result = part1(EXAMPLE2);
        assert_eq!(result, 11687500);
    }

    #[test]
    fn test_part1_solution() {
        let result = part1(&read_input_file());
        assert_eq!(result, 836127690);
    }

    // #[test]
    // fn test_part2_example() {
    //     let result = part2(EXAMPLE);
    //     assert_eq!(result, 0);
    // }

    // #[test]
    // fn test_part2_solution() {
    //     let result = part2(&read_input_file());
    //     assert_eq!(result, 0);
    // }
}
