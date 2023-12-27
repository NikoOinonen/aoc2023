use core::num;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
    ops::Not,
    vec,
};

use aoc2023::filter_input_lines;

use super::Problem;

trait Module: Debug {
    fn send_pulses(&mut self, input_pulse: Pulse) -> Vec<Pulse>;
    fn as_con(&mut self) -> Option<&mut Conjunction> {
        None
    }
    fn outputs(&self) -> Vec<String>;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    High,
    Low,
}

impl Not for State {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            State::High => State::Low,
            State::Low => State::High,
        }
    }
}

#[derive(Debug)]
struct Pulse {
    state: State,
    from: String,
    to: String,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -{:?}-> {}", self.from, self.state, self.to)
    }
}

#[derive(Debug)]
struct FlipFlop {
    state: State,
    name: String,
    outputs: Vec<String>,
}

impl Module for FlipFlop {
    fn send_pulses(&mut self, input_pulse: Pulse) -> Vec<Pulse> {
        if input_pulse.state == State::Low {
            self.state = !self.state;
            self.outputs
                .iter()
                .map(|target| Pulse {
                    state: self.state,
                    from: self.name.clone(),
                    to: target.to_owned(),
                })
                .collect()
        } else {
            vec![]
        }
    }
    fn outputs(&self) -> Vec<String> {
        self.outputs.clone()
    }
}

#[derive(Debug)]
struct Conjunction {
    inputs: HashMap<String, State>,
    name: String,
    outputs: Vec<String>,
}

impl Module for Conjunction {
    fn send_pulses(&mut self, input_pulse: Pulse) -> Vec<Pulse> {
        self.inputs.insert(input_pulse.from, input_pulse.state);
        let mut output_state = State::Low;
        for (_, state) in self.inputs.iter() {
            if *state == State::Low {
                output_state = State::High;
                break;
            }
        }
        self.outputs
            .iter()
            .map(|target| Pulse {
                state: output_state,
                from: self.name.clone(),
                to: target.to_owned(),
            })
            .collect()
    }
    fn as_con(&mut self) -> Option<&mut Conjunction> {
        Some(self)
    }
    fn outputs(&self) -> Vec<String> {
        self.outputs.clone()
    }
}

#[derive(Debug)]
struct Broadcaster {
    outputs: Vec<String>,
}

impl Module for Broadcaster {
    fn send_pulses(&mut self, input_pulse: Pulse) -> Vec<Pulse> {
        self.outputs
            .iter()
            .map(|target| Pulse {
                state: input_pulse.state,
                from: "broadcaster".to_string(),
                to: target.to_owned(),
            })
            .collect()
    }
    fn outputs(&self) -> Vec<String> {
        self.outputs.clone()
    }
}

#[derive(Debug)]
struct Button;

pub struct Day;

impl Problem for Day {
    fn part_one(&self, input: &str) -> String {
        let mut modules = get_modules(input);

        let mut pulses: VecDeque<Pulse> = VecDeque::new();
        let mut num_low_pulses = 0;
        let mut num_high_pulses = 0;
        for _ in 0..1000 {
            pulses.push_back(Pulse {
                state: State::Low,
                from: "button".to_string(),
                to: "broadcaster".to_string(),
            });
            while let Some(pulse) = pulses.pop_front() {
                if pulse.state == State::Low {
                    num_low_pulses += 1;
                } else {
                    num_high_pulses += 1;
                }
                if let Some(module) = modules.get_mut(&pulse.to) {
                    for pulse in module.send_pulses(pulse) {
                        pulses.push_back(pulse);
                    }
                }
            }
        }

        let value = num_low_pulses * num_high_pulses;
        println!("{num_low_pulses}, {num_high_pulses}");
        println!("{value}");
        format!("{value}")
    }

    fn part_two(&self, input: &str) -> String {
        let mut modules = get_modules(input);

        let rx_con = modules.iter().find(|(_, module)| module.outputs().contains(&"rx".to_owned())).unwrap().0.clone();
        let con_inputs: Vec<String> = modules.iter().filter_map(|(name, module)| {
            if module.outputs().contains(&rx_con) {
                Some(name.to_owned())
            } else {
                None
            }
        }).collect();
        println!("{con_inputs:?}");

        let mut pulses: VecDeque<Pulse> = VecDeque::new();
        let mut num_presses = 0;
        let mut con_cycles = HashMap::new();
        loop {
            pulses.push_back(Pulse {
                state: State::Low,
                from: "button".to_string(),
                to: "broadcaster".to_string(),
            });
            num_presses += 1;
            if (num_presses % 100) == 0 {
                println!("{num_presses}");
            }
            while let Some(pulse) = pulses.pop_front() {
                if pulse.to == rx_con && pulse.state == State::High && !con_cycles.contains_key(&pulse.to) {
                    con_cycles.insert(pulse.from.to_owned(), num_presses);
                }
                if let Some(module) = modules.get_mut(&pulse.to) {
                    for pulse in module.send_pulses(pulse) {
                        pulses.push_back(pulse);
                    }
                }
            }
            if con_inputs.iter().all(|name| con_cycles.contains_key(name)) {
                break;
            }
        }

        println!("{con_cycles:?}");
        let num_presses: u64 = con_cycles.iter().map(|(_, val)| *val as u64).product();

        println!("{num_presses}");
        format!("{num_presses}")
    }
}

fn get_modules(input: &str) -> HashMap<String, Box<dyn Module>> {
    let lines = filter_input_lines(input);
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    for line in lines {
        let (inp, out) = line.split_once("->").unwrap();
        let outputs: Vec<String> = out.split(',').map(|s| s.trim().to_string()).collect();
        let (module, name): (Box<dyn Module>, String) = match inp.trim() {
            "broadcaster" => (Box::new(Broadcaster { outputs }), "broadcaster".to_string()),
            s => {
                let name = s[1..].to_string();
                match s.chars().nth(0).unwrap() {
                    '%' => (
                        Box::new(FlipFlop {
                            state: State::Low,
                            name: name.clone(),
                            outputs,
                        }),
                        name,
                    ),
                    '&' => (
                        Box::new(Conjunction {
                            inputs: HashMap::new(),
                            name: name.clone(),
                            outputs,
                        }),
                        name,
                    ),
                    _ => panic!(),
                }
            }
        };
        modules.insert(name, module);
    }

    let names: Vec<(String, Vec<String>)> = modules.iter().map(|(name, module)| (name.to_owned(), module.outputs())).collect();
    for (name, outputs) in names {
        for output in outputs {
            if let Some(module) = modules.get_mut(&output) {
                if let Some(con) = module.as_con() {
                    con.inputs.insert(name.to_owned(), State::Low);
                }
            }
        }
    }

    modules
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a";
        let value = Day.part_one(input);
        assert_eq!(value, "32000000");

        let input = "
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output";
        let value = Day.part_one(input);
        assert_eq!(value, "11687500");
    }

}
