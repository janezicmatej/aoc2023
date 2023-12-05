use std::{
    cmp::{max, min},
    str::FromStr,
};

use aoc::parsers::to_vec;

type PairRange = (u64, u64);

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

    fn contains_any(&self, (s, r): PairRange) -> bool {
        s < self.source + self.range && s + r > self.source
    }

    fn map(&self, n: u64) -> u64 {
        let shift = n - self.source;
        self.destination + shift
    }

    fn split_range(&self, (s, r): PairRange) -> [Option<PairRange>; 3] {
        let mut fences = [
            s,
            s + r,
            max(self.source, s),
            min(self.source + self.range, s + r),
        ];
        fences.sort();

        let mut v = Vec::new();

        for i in 0..3 {
            let f = fences[i];
            let nf = fences[i + 1];
            if f != nf {
                v.push(Some((f, nf - f)))
            } else {
                v.push(None)
            }
        }

        v[..3].try_into().unwrap()
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
                if map.contains_any((s, r)) {
                    let [pre, to_map, post] = map.split_range((s, r));
                    let to_map = to_map.unwrap();
                    let mapped = (map.map(to_map.0), to_map.1);
                    new_ranges.push(mapped);
                    for r in [pre, post].into_iter().flatten() {
                        seeds_ranges.push(r);
                    }
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
