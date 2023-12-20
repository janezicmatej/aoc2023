use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use aoc::lcm;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Module {
    Broadcaster,
    FlipFlop(bool),
    Conjuction,
    Output,
}

#[derive(Debug)]
struct ParseModuleError;

impl FromStr for Module {
    type Err = ParseModuleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "%" => Module::FlipFlop(false),
            "&" => Module::Conjuction,
            "b" => Module::Broadcaster,
            _ => return Err(ParseModuleError),
        })
    }
}

#[derive(Debug)]
struct Node {
    index: usize,
    module: Module,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
}

fn parse_input(input: &str) -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut mapper: HashMap<String, usize> = HashMap::new();

    let output = Node {
        index: 0,
        module: Module::Output,
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    nodes.push(output);

    for line in input.lines() {
        let (from, _) = line.split_once(" -> ").unwrap();
        let (module, name) = from.split_at(1);
        let module: Module = module.parse().unwrap();

        *mapper.entry(name.to_string()).or_default() = nodes.len();

        let l = Node {
            module,
            index: nodes.len(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        };

        nodes.push(l);
    }

    for line in input.lines() {
        let (from, to) = line.split_once(" -> ").unwrap();
        let (_, name) = from.split_at(1);

        let index = mapper[name];
        for destination in to.split(", ") {
            let to_index = *mapper.get(destination).unwrap_or(&0);
            nodes[index].outputs.push(to_index);
            nodes[to_index].inputs.push(index);
        }
    }

    nodes
}

pub fn cycle(input: &str, cycle_len: usize) -> (usize, usize) {
    let mut nodes = parse_input(input);

    let mut graph = vec![vec![false; nodes.len()]; nodes.len()];

    let mut highs = 0;
    let mut lows = 0;

    let broadcaster = nodes
        .iter()
        .find(|x| x.module == Module::Broadcaster)
        .unwrap()
        .index;

    let mut hm = HashMap::new();

    for i in 1..=cycle_len {
        let mut stack = VecDeque::from([(broadcaster, false)]);

        while let Some((index, signal)) = stack.pop_front() {
            if signal {
                highs += 1;
            } else {
                lows += 1;
            }

            let node = &nodes[index];
            let mut new_module = None;

            match node.module {
                Module::Output => (),
                Module::Broadcaster => {
                    for dest_index in node.outputs.iter() {
                        stack.push_back((*dest_index, false));
                    }
                }
                Module::FlipFlop(high) => {
                    if !signal {
                        let signal = !high;

                        for dest_index in nodes[index].outputs.iter() {
                            graph[index][*dest_index] = signal;
                            stack.push_back((*dest_index, signal));
                        }

                        new_module = Some(Module::FlipFlop(signal));
                    }
                }
                Module::Conjuction => {
                    let mut all = true;

                    for in_index in nodes[index].inputs.iter() {
                        all &= graph[*in_index][index];
                    }

                    for dest_index in nodes[index].outputs.iter().copied() {
                        if dest_index == 0 {
                            for in_index in nodes[index].inputs.iter() {
                                if graph[*in_index][index] {
                                    hm.entry(*in_index).or_insert(i);
                                }
                            }
                        }
                        graph[index][dest_index] = !all;
                        stack.push_back((dest_index, !all));
                    }
                }
            }

            let node = &mut nodes[index];
            if let Some(module) = new_module {
                node.module = module;
            }
        }
    }

    (highs * lows, hm.values().copied().reduce(lcm).unwrap_or(0))
}
pub fn part_one(input: &str) -> Option<usize> {
    Some(cycle(input, 1000).0)
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(cycle(input, 10_000).1)
}

aoc::solution!(20);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one_first() {
        assert_eq!(
            part_one(&aoc::template::read_file_part("examples", 20, 1)),
            Some(32000000)
        );
    }
    #[test]
    fn test_part_one_second() {
        assert_eq!(
            part_one(&aoc::template::read_file_part("examples", 20, 2)),
            Some(11687500)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file_part("examples", 20, 2)),
            Some(1)
        );
    }
}
