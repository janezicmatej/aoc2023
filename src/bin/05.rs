use std::{cmp::min, str::FromStr};

use aoc::parsers::to_vec;

#[derive(Debug)]
struct Mapping {
    destination: u64,
    source: u64,
    range: u64,
}

struct ParseMappingError;

impl FromStr for Mapping {
    type Err = ParseMappingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<u64> = to_vec(s, ' ');

        Ok(Self {
            destination: nums[0],
            source: nums[1],
            range: nums[2],
        })
    }
}

impl Mapping {
    fn contains(&self, n: u64) -> bool {
        n >= self.source && n < self.source + self.range
    }

    fn contains_any(&self, s: u64, r: u64) -> bool {
        s < self.source + self.range && s + r > self.source
    }

    fn map(&self, n: u64) -> u64 {
        debug_assert!(self.contains(n));
        let shift = n - self.source;
        self.destination + shift
    }

    fn map_range(&self, s: u64, r: u64) -> ((u64, u64), Vec<(u64, u64)>) {
        debug_assert!(self.contains_any(s, r));
        let shift = {
            if self.contains(s) {
                s - self.source
            } else {
                0
            }
        };
        let neg_shift = {
            if self.contains(s) {
                0
            } else {
                self.source - s
            }
        };
        let dest_start = self.destination + shift;
        let dest_range = min(r - neg_shift, self.range - shift);
        let mut rest = Vec::new();

        if !self.contains(s) {
            rest.push((s, self.source - s));
        }

        if r - neg_shift > dest_range {
            rest.push((self.source + self.range, r - dest_range))
        }

        ((dest_start, dest_range), rest)
    }
}

fn parse_map(maps: &str) -> Vec<Vec<Mapping>> {
    let mut res = Vec::new();

    for mapper in maps.split("\n\n") {
        res.push(
            mapper
                .lines()
                .skip(1)
                .filter_map(|m| m.parse().ok())
                .collect(),
        )
    }

    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let (first, rest) = input.split_once("\n\n").unwrap();
    let maps = parse_map(rest);
    let seeds = to_vec::<u64, _>(first.strip_prefix("seeds: ").unwrap(), ' ');

    let mut m = u64::MAX;

    for seed in seeds.iter() {
        let mut s = *seed;
        'maps: for map in maps.iter() {
            for inner in map.iter() {
                if inner.contains(s) {
                    s = inner.map(s);
                    continue 'maps;
                }
            }
        }

        m = min(m, s)
    }

    Some(m)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (first, rest) = input.split_once("\n\n").unwrap();
    let maps = parse_map(rest);
    let seeds = to_vec::<u64, _>(first.strip_prefix("seeds: ").unwrap(), ' ');
    let mut seeds_ranges: Vec<_> = Vec::new();
    for s in seeds.chunks(2) {
        seeds_ranges.push((s[0], s[1]))
    }

    for mapping in maps.iter() {
        let mut new_ranges = Vec::new();

        'queue: while let Some((s, r)) = seeds_ranges.pop() {
            for map in mapping {
                if map.contains_any(s, r) {
                    let (rng, rest) = map.map_range(s, r);
                    new_ranges.push(rng);
                    seeds_ranges.extend_from_slice(&rest);
                    continue 'queue;
                }
            }

            new_ranges.push((s, r));
        }

        seeds_ranges = new_ranges;
    }

    seeds_ranges.iter().map(|(x, _)| *x).min()
}

aoc::solution!(5);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::template::read_file("examples", 5)), Some(35));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 5)), Some(46));
    }
}
