use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("part 1 result: {:?}", part1(&read_input_file()));
    println!("part 2 result: {:?}", part2(&read_input_file(), RX));
}

fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

const BROADCAST: &'static str = "broadcaster";
const BUTTON: &'static str = "button";
const OUTPUT: &'static str = "output";
const RX: &'static str = "rx";
const OUTPUTS: [&'static str; 2] = [OUTPUT, RX];

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
        let pulses: Vec<(Pulse, String, String)> =
            (0..times).flat_map(|_| self.push_button()).collect();

        let high_count = pulses
            .iter()
            .filter(|(pulse, _source, _destination)| *pulse == Pulse::High)
            .count();

        let low_count = pulses
            .iter()
            .filter(|(pulse, _source, _destination)| *pulse == Pulse::Low)
            .count();

        high_count * low_count
    }

    fn num_presses_until_low_pulse(&mut self, output_name: &str) -> usize {
        let inputs_for_output = Configuration::inputs_to(&self.modules, &output_name.to_string());

        // ensure there is exactly one conjunction wired up to the output
        assert_eq!(inputs_for_output.len(), 1);
        let conj = &inputs_for_output[0];
        assert_eq!(
            self.modules.get(conj).unwrap().kind,
            ModuleType::Conjunction
        );

        let mut awaiting_first_high_pulse: HashSet<String> =
            Configuration::inputs_to(&self.modules, &conj)
                .into_iter()
                .collect();
        let mut num_presses = 0;
        let mut first_high_pulses = vec![];

        while !awaiting_first_high_pulse.is_empty() {
            let pulses = self.push_button();
            num_presses += 1;

            for (p, source, destination) in pulses {
                if p == Pulse::High
                    && destination == *conj
                    && awaiting_first_high_pulse.contains(&source)
                {
                    first_high_pulses.push(num_presses);
                    awaiting_first_high_pulse.remove(&source);
                }
            }
        }

        // least common multiple of the first high presses of each input
        lcm(&first_high_pulses)
    }

    fn push_button(&mut self) -> Vec<(Pulse, String, String)> {
        let mut pulses_to_process: VecDeque<(Pulse, String, String)> = VecDeque::new();
        pulses_to_process.push_back((Pulse::Low, BUTTON.to_string(), BROADCAST.to_string()));

        let mut pulses_processed = vec![];

        while let Some(curr) = pulses_to_process.pop_front() {
            pulses_processed.push(curr.clone());

            let (pulse, source, destination) = curr;

            match self.modules.get(&destination) {
                Some(module) => {
                    let new_pulses = module.process(pulse, source, &mut self.module_state);
                    for p in new_pulses {
                        pulses_to_process.push_back(p);
                    }
                }
                None => {
                    // output destination, do nothing
                    assert_eq!(OUTPUTS.contains(&destination.as_str()), true);
                }
            }
        }

        pulses_processed
    }

    fn inputs_to(modules: &HashMap<String, Module>, name: &String) -> Vec<String> {
        modules
            .iter()
            .filter(|(_, module)| module.destinations.contains(name))
            .map(|(name, _)| name)
            .cloned()
            .collect()
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
            assert_eq!(name, BROADCAST);
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Eq, PartialEq)]
struct ModuleState {
    conjunctions: HashMap<String, HashMap<String, Pulse>>,
    flip_flops: HashMap<String, bool>,
}

impl ModuleState {
    fn new(modules: &HashMap<String, Module>) -> Self {
        // initialize as maps of inbound connections to low pulse
        let conjunctions: HashMap<String, HashMap<String, Pulse>> = modules
            .iter()
            .filter(|(_, module)| module.kind == ModuleType::Conjunction)
            .map(|(_, module)| {
                let memory: HashMap<String, Pulse> =
                    Configuration::inputs_to(&modules, &module.name)
                        .into_iter()
                        .map(|name| (name, Pulse::Low))
                        .collect();
                (module.name.clone(), memory)
            })
            .collect();

        // initialize as false
        let flip_flops: HashMap<String, bool> = modules
            .iter()
            .filter(|(_, module)| module.kind == ModuleType::FlipFlop)
            .map(|(_, module)| (module.name.clone(), false))
            .collect();

        ModuleState {
            conjunctions,
            flip_flops,
        }
    }

    fn get_conjunction(&mut self, module: &String) -> &mut HashMap<String, Pulse> {
        self.conjunctions.get_mut(module).unwrap()
    }

    fn get_flip_flop(&mut self, module: &String) -> &mut bool {
        self.flip_flops.get_mut(module).unwrap()
    }
}

fn lcm(values: &[usize]) -> usize {
    values
        .iter()
        .cloned()
        .reduce(|acc, x| num::integer::lcm(acc, x))
        .unwrap()
}

fn part1(input: &str) -> usize {
    let mut config = Configuration::parse(input);
    config.count_pulses_for_presses(1000)
}

fn part2(input: &str, output_name: &str) -> usize {
    let mut config = Configuration::parse(input);
    config.num_presses_until_low_pulse(output_name)
}

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

    #[test]
    fn test_part2_example2() {
        let result = part2(EXAMPLE2, OUTPUT);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part2_solution() {
        let result = part2(&read_input_file(), RX);
        assert_eq!(result, 240914003753369);
    }
}
