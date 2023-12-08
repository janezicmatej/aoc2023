use std::collections::HashMap;

use aoc::lcm;

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = HashMap::new();

    let (directions, map_data) = input.split_once("\n\n")?;
    for line in map_data.lines() {
        let (k, v) = line.split_once(" = ")?;
        let v = v.strip_prefix('(')?;
        let v = v.strip_suffix(')')?;
        let (l, r) = v.split_once(", ")?;

        *map.entry(k).or_default() = (l, r);
    }

    let mut counter = 0;
    let mut loc = "AAA";

    for d in directions.chars().cycle() {
        if loc == "ZZZ" {
            break;
        }

        counter += 1;
        let (l, r) = map[loc];

        match d {
            'L' => loc = l,
            'R' => loc = r,
            _ => unimplemented!(),
        }
    }

    Some(counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = HashMap::new();

    let (directions, map_data) = input.split_once("\n\n")?;
    for line in map_data.lines() {
        let (k, v) = line.split_once(" = ")?;
        let v = v.strip_prefix('(')?;
        let v = v.strip_suffix(')')?;
        let (l, r) = v.split_once(", ")?;

        *map.entry(k).or_default() = (l, r);
    }

    let mut res = 1;

    for k in map.keys().filter(|x| x.ends_with('A')) {
        let mut location = *k;

        for (i, d) in directions.chars().cycle().enumerate() {
            let (l, r) = map[location];

            location = match d {
                'L' => l,
                'R' => r,
                _ => unimplemented!(),
            };

            if location.ends_with('Z') {
                res = lcm(res, i + 1);
                break;
            }
        }
    }

    Some(res as u64)
}

aoc::solution!(8);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&aoc::template::read_file_part("examples", 8, 1)),
            Some(6)
        );
    }
    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&aoc::template::read_file_part("examples", 8, 2)),
            Some(6)
        );
    }
}
