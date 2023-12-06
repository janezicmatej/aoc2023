use std::{
    cmp::{max, min},
    fmt::Debug,
    ops::RangeInclusive,
    str::FromStr,
};

use aoc::parsers::to_vec;

trait RangeInclusiveExt {
    fn overlaps(&self, other: &Self) -> bool;
}

impl<T> RangeInclusiveExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn build_range(start: u64, range: u64) -> RangeInclusive<u64> {
    start..=(start + range - 1)
}

#[derive(Debug)]
struct Mapping {
    pub source: RangeInclusive<u64>,
    pub destination: RangeInclusive<u64>,
}

struct ParseMappingError;

impl FromStr for Mapping {
    type Err = ParseMappingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<u64> = to_vec(s, ' ');

        Ok(Self {
            destination: build_range(nums[0], nums[2]),
            source: build_range(nums[1], nums[2]),
        })
    }
}

impl Mapping {
    fn map(&self, n: u64) -> u64 {
        let shift = n - self.source.start();
        self.destination.start() + shift
    }

    fn map_range(&self, r: RangeInclusive<u64>) -> RangeInclusive<u64> {
        self.map(*r.start())..=(self.map(*r.end()))
    }

    fn split_range(&self, r: RangeInclusive<u64>) -> [Option<RangeInclusive<u64>>; 3] {
        let mut fences = [
            *r.start(),
            *r.end() + 1,
            max(*self.source.start(), *r.start()),
            min(*self.source.end() + 1, *r.end() + 1),
        ];

        fences.sort();

        const ARRAY_REPEAT_VALUE: Option<RangeInclusive<u64>> = None;
        let mut v = [ARRAY_REPEAT_VALUE; 3];

        for i in 0..3 {
            let f = fences[i];
            let nf = fences[i + 1];
            if f != nf {
                v[i] = Some(f..=(nf - 1))
            }
        }
        v
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
                if inner.source.contains(&s) {
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
    let mut seeds_ranges: Vec<_> = to_vec::<u64, _>(first.strip_prefix("seeds: ").unwrap(), ' ')
        .chunks(2)
        .map(|x| build_range(x[0], x[1]))
        .collect();

    for mapping in maps.iter() {
        let mut new_ranges = Vec::new();

        'queue: while let Some(rng) = seeds_ranges.pop() {
            for map in mapping {
                if map.source.overlaps(&rng) {
                    let [pre, to_map, post] = map.split_range(rng);
                    new_ranges.push(map.map_range(to_map.unwrap()));
                    for r in [pre, post].into_iter().flatten() {
                        seeds_ranges.push(r);
                    }
                    continue 'queue;
                }
            }

            new_ranges.push(rng);
        }

        seeds_ranges = new_ranges;
    }

    seeds_ranges
        .iter()
        .map(RangeInclusive::start)
        .min()
        .copied()
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
