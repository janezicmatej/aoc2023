use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Module {
    Broadcaster,
    FlipFlop,
    Conjuction,
}

#[derive(Debug)]
struct ParseModuleError;

impl FromStr for Module {
    type Err = ParseModuleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "%" => Module::FlipFlop,
            "&" => Module::Conjuction,
            _ => Module::Broadcaster,
        })
    }
}

#[derive(Debug)]
struct Location {
    module: Module,
    inputs: HashMap<String, Option<bool>>,
    outputs: Vec<String>,
}

fn parse_input(input: &str) -> (Vec<Location>, HashMap<String, usize>) {
    let mut locations = Vec::new();
    let mut mapper: HashMap<String, usize> = HashMap::new();

    for line in input.lines() {
        let (from, to) = line.split_once(" -> ").unwrap();
        let (module, mut name) = from.split_at(1);
        let module: Module = module.parse().unwrap();
        if module == Module::Broadcaster {
            name = from;
        }

        let l = Location {
            module,
            inputs: HashMap::new(),
            outputs: to.split(", ").map(String::from).collect(),
        };

        locations.push(l);
        *mapper.entry(name.to_string()).or_default() = locations.len() - 1;
    }
    for line in input.lines() {
        let (from, to) = line.split_once(" -> ").unwrap();
        let (_, name) = from.split_at(1);

        for destination in to.split(", ") {
            let index = mapper[name];
            locations[index]
                .inputs
                .entry(destination.to_string())
                .or_default();
        }
    }

    (locations, mapper)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (locations, mapper) = parse_input(input);

    let mut high = 0;
    let mut low = 0;

    for _ in 0..1000 {
        let mut stack = vec!["broadcaster".to_string()];
        while !stack.is_empty() {
            let mut new_stack = Vec::new();

            while let Some(s) = stack.pop() {
                let index = mapper[&s];
                let loc = &locations[index];

                match loc.module {
                    Module::Broadcaster => {
                        for dest in loc.outputs.iter() {
                            let index = mapper[dest];
                            let loc = &locations[index];
                        }
                    }
                }
            }

            stack = new_stack;
        }
    }

    Some(high * low)
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
