use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Brick {
    from: (usize, usize, usize),
    to: (usize, usize, usize),
}

#[derive(Debug)]
struct ParseBrickError;

impl FromStr for Brick {
    type Err = ParseBrickError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once('~').ok_or(ParseBrickError)?;

        let sorted: Vec<_> = first
            .split(',')
            .filter_map(|y| y.parse().ok())
            .zip(second.split(',').filter_map(|y| y.parse().ok()))
            .map(|(x, y)| if x < y { (x, y) } else { (y, x) })
            .collect();
        let from = (sorted[0].0, sorted[1].0, sorted[2].0);
        let to = (sorted[0].1, sorted[1].1, sorted[2].1);

        Ok(Self { from, to })
    }
}

impl Brick {
    fn height(&self) -> usize {
        self.to.2 - self.from.2 + 1
    }

    fn points(&self) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        (self.from.0..=self.to.0).flat_map(move |x| {
            (self.from.1..=self.to.1)
                .flat_map(move |y| (self.from.2..=self.to.2).map(move |z| (x, y, z)))
        })
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.from
            .2
            .cmp(&other.from.2)
            .then((self.from.0, self.from.1).cmp(&(other.from.0, other.from.1)))
    }
}

#[derive(Debug, Clone)]
struct Tower {
    bricks: Vec<Brick>,
}

impl Tower {
    fn new(mut bricks: Vec<Brick>) -> Self {
        bricks.sort();
        let mut ret = Self { bricks };
        ret.compress(None);
        ret
    }

    fn compress(&mut self, skip: Option<usize>) -> usize {
        let mut heights: HashMap<(usize, usize), usize> = HashMap::new();
        let mut moved = 0;

        for brick in self
            .bricks
            .iter_mut()
            .enumerate()
            .filter(|(i, _)| *i != skip.unwrap_or(usize::MAX))
            .map(|(_, x)| x)
        {
            let height = brick.height();
            let new_height = brick
                .points()
                .map(|(x, y, _)| *heights.entry((x, y)).or_default())
                .max()
                .unwrap();

            for (x, y, _) in brick.points() {
                *heights.get_mut(&(x, y)).unwrap() = new_height + height;
            }

            if new_height + 1 == brick.from.2 {
                continue;
            }

            if skip.is_none() {
                brick.from.2 = new_height + 1;
                brick.to.2 = new_height + height;
            }

            moved += 1;
        }

        moved
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut tower = Tower::new(input.lines().filter_map(|x| x.parse().ok()).collect());

    Some(
        (0..tower.bricks.len())
            .map(|i| tower.compress(Some(i)))
            .filter(|m| *m == 0)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut tower = Tower::new(input.lines().filter_map(|x| x.parse().ok()).collect());

    Some(
        (0..tower.bricks.len())
            .map(|i| tower.compress(Some(i)))
            .sum(),
    )
}

aoc::solution!(22);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::template::read_file("examples", 22)), Some(5));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 22)), Some(7));
    }
}
