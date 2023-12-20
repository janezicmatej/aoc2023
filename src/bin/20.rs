use std::{
    collections::{HashMap, VecDeque},
    mem::swap,
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Module {
    Broadcaster,
    FlipFlop(bool),
    Conjuction,
}

#[derive(Debug)]
struct ParseModuleError;

impl FromStr for Module {
    type Err = ParseModuleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "%" => Module::FlipFlop(false),
            "&" => Module::Conjuction,
            _ => Module::Broadcaster,
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

    for line in input.lines() {
        let (from, _) = line.split_once(" -> ").unwrap();
        let (module, mut name) = from.split_at(1);
        let module: Module = module.parse().unwrap();
        if module == Module::Broadcaster {
            name = from;
        }

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
        let (module, mut name) = from.split_at(1);
        let module: Module = module.parse().unwrap();
        if module == Module::Broadcaster {
            name = from;
        }

        let index = mapper[name];
        for destination in to.split(", ") {
            let to_index = mapper[destination];
            nodes[index].outputs.push(to_index);
            nodes[to_index].inputs.push(index);
        }
    }

    nodes
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut nodes = parse_input(input);

    let mut graph = vec![vec![None; nodes.len()]; nodes.len()];

    let mut highs = 0;
    let mut lows = 0;

    let broadcaster = nodes
        .iter()
        .find(|x| x.module == Module::Broadcaster)?
        .index;

    for _ in 0..1000 {
        let mut stack = VecDeque::from([broadcaster]);
        while !stack.is_empty() {
            let mut new_stack = VecDeque::new();
            let mut new_graph = graph.clone();

            while let Some(index) = stack.pop_front() {
                let node = &nodes[index];
                let mut new_module = None;

                match node.module {
                    Module::Broadcaster => {
                        for dest_index in node.outputs.iter() {
                            new_graph[index][*dest_index] = Some(false);
                            new_stack.push_back(*dest_index);
                            lows += 1;
                        }
                    }
                    Module::FlipFlop(high) => {
                        let mut swapper = None;
                        for in_index in nodes[index].inputs.iter() {
                            let value = &mut graph[*in_index][index];
                            if value.is_some() {
                                debug_assert!(swapper.is_none());
                                swap(&mut swapper, value);
                            }
                        }

                        if !swapper.unwrap() {
                            let signal = !high;

                            for dest_index in nodes[index].outputs.iter() {
                                new_graph[index][*dest_index] = Some(signal);
                                new_stack.push_back(*dest_index);
                                if signal {
                                    highs += 1;
                                } else {
                                    lows += 1;
                                }
                            }

                            new_module = Some(Module::FlipFlop(signal));
                        }
                    }
                    Module::Conjuction => {
                        let mut all = true;

                        for in_index in nodes[index].inputs.iter() {
                            let value = graph[*in_index][index];
                            all &= value.unwrap_or(false);
                        }

                        for dest_index in nodes[index].outputs.iter() {
                            new_graph[index][*dest_index] = Some(!all);
                            new_stack.push_back(*dest_index);
                            if !all {
                                highs += 1;
                            } else {
                                lows += 1;
                            }
                        }
                    }
                }

                let node = &mut nodes[index];
                if let Some(module) = new_module {
                    node.module = module;
                }
            }

            stack = new_stack;
            graph = new_graph;
        }
    }

    Some(highs * lows)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

aoc::solution!(20);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::template::read_file("examples", 20)), None);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 20)), None);
    }
}
