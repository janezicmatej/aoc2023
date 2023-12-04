use std::str::FromStr;

use aoc::parsers::to_vec;

struct ParseCardError;

struct Card {
    winning: Vec<u32>,
    numbers: Vec<u32>,
}

impl FromStr for Card {
    type Err = ParseCardError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, rest) = s.split_once(": ").unwrap();
        let (win, my) = rest.split_once(" | ").unwrap();

        let winning: Vec<u32> = to_vec(win, ' ');
        let numbers: Vec<u32> = to_vec(my, ' ');

        Ok(Card { winning, numbers })
    }
}

impl Card {
    fn n_matches(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }
    fn score(&self) -> u32 {
        let c = self.n_matches();

        if c == 0 {
            return 0;
        }

        2_u32.pow((self.n_matches() - 1) as u32)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| l.parse::<Card>().ok())
            .map(|g| g.score())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards: Vec<Card> = to_vec(input, '\n');
    let mut multiples = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        for j in 0..card.n_matches() {
            multiples[i + j + 1] += multiples[i];
        }
    }

    Some(multiples.iter().sum::<u32>())
}

aoc::solution!(4);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::template::read_file("examples", 4)), Some(13));
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::template::read_file("examples", 4)), Some(30));
    }
}
